use std::cmp::Ordering;

use crate::{components::{trajectory_component::orbit::Orbit, ComponentType}, constants::PREDICTION_TIME_STEP, state::State, storage::entity_allocator::Entity, systems::trajectory_prediction::util::get_parallel_entities};

use self::bounding::{find_encounter_bounds, EncounterBoundType};

use super::util::{Encounter, EncounterType};

mod bounding;

fn angle_window_to_time_window(orbit: &Orbit, window: (f64, f64)) -> (f64, f64) {
    (orbit.get_first_periapsis_time() + orbit.get_time_since_first_periapsis(window.0), orbit.get_first_periapsis_time() + orbit.get_time_since_first_periapsis(window.1))
}

fn solve_for_encounter(orbit: &Orbit, other_orbit: &Orbit, start_time: f64, end_time: f64) -> Option<f64> {
    let soi = other_orbit.get_sphere_of_influence();
    let f = |time: f64| {
        let position = orbit.get_theta_from_time(time);
        let other_position = other_orbit.get_theta_from_time(time);
        (position - other_position).abs() - soi
    };
    let mut time = start_time;
    let mut previous_distance = f64::MAX;
    while time < end_time {
        let distance = f(time);
        if distance.is_sign_negative() && previous_distance.is_sign_positive() {
            return Some(time);
        }
        previous_distance = distance;
        time += PREDICTION_TIME_STEP;
    }
    None
}

#[derive(Debug)]
struct TimeWindow<'a> {
    orbit: &'a Orbit,
    other_orbit: &'a Orbit,
    other_entity: Entity,
    window: (f64, f64),
}

impl<'a> TimeWindow<'a> {
    pub fn new(orbit: &'a Orbit, other_orbit: &'a Orbit, other_entity: Entity, window: (f64, f64)) -> Self {
        Self { orbit, other_orbit, other_entity, window }
    }

    fn get_soonest_time(&self) -> f64 {
        f64::min(self.window.0, self.window.1)
    }

    fn get_latest_time(&self) -> f64 {
        f64::max(self.window.0, self.window.1)
    }

    fn increment_by_period(&mut self) {
        self.window.0 += self.orbit.get_period().unwrap();
        self.window.1 += self.orbit.get_period().unwrap();
    }

    // Order by the soonest time we'll have to think about an encounter
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.get_soonest_time() < other.get_soonest_time() {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

fn get_initial_bounds(state: &State, entity: Entity, start_time: f64) -> (Vec<Entity>, Vec<TimeWindow>) {
    let orbit = state.get_trajectory_component(entity).get_end_segment().as_orbit();
    let can_enter = &state.get_entities(vec![ComponentType::TrajectoryComponent]);
    let parallel_entities = get_parallel_entities(state, can_enter, entity);
    let mut unbounded = vec![];
    let mut time_bounds = vec![];
    for other_entity in parallel_entities {
        let other_orbit = state.get_trajectory_component(other_entity).get_segment_at_time(start_time).as_orbit();
        let bounds = find_encounter_bounds(orbit, other_orbit);
        match bounds {
            EncounterBoundType::NoEncounters => (),
            EncounterBoundType::NoBounds => unbounded.push(other_entity),
            EncounterBoundType::One(angle_window) => {
                let time_window = angle_window_to_time_window(&orbit, angle_window);
                //TODO the unwrap will cause the program to crash encountering hyperbolic case
                time_bounds.push(TimeWindow::new(orbit, other_orbit, other_entity, time_window))
            }
            EncounterBoundType::Two(angle_window_1, angle_window_2) => {
                let time_window_1 = angle_window_to_time_window(&orbit, angle_window_1);
                let time_window_2 = angle_window_to_time_window(&orbit, angle_window_2);
                //TODO the unwrap will cause the program to crash encountering hyperbolic case
                time_bounds.push(TimeWindow::new(orbit, other_orbit, other_entity, time_window_1));
                time_bounds.push(TimeWindow::new(orbit, other_orbit, other_entity, time_window_2));
            }
        }
    }
    for bound in &mut time_bounds {
        while bound.get_soonest_time() < start_time && bound.get_latest_time() < start_time {
            bound.increment_by_period()
        }
    }
    time_bounds.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    (unbounded, time_bounds)
}


pub fn find_next_encounter(state: &State, entity: Entity, start_time: f64, end_time: f64) -> () {
    let (unbounded, mut time_bounds) = get_initial_bounds(state, entity, start_time);

    if time_bounds.is_empty() {
        // brute force
    }

    let mut start_time = time_bounds.first().unwrap().get_soonest_time();
    let mut end_time = end_time;
    let mut soonest_encounter: Option<Encounter> = None;

    loop {
        if start_time > end_time {
            break;
        }

        let first = time_bounds.first().unwrap();
        let new_start_time = first.get_soonest_time();
        
        // Check if window contains encounters
        if let Some(encounter_time) = solve_for_encounter(first.orbit, first.other_orbit, first.get_soonest_time(), f64::min(first.get_latest_time(), end_time)) {

            // If so update soonest encounter + end time if new encounter occurs sooner
            let should_update = if let Some(soonest_encounter) = &mut soonest_encounter {
                encounter_time < soonest_encounter.get_time()
            } else {
                true
            };
            if should_update {
                soonest_encounter = Some(Encounter::new(EncounterType::Entrance, entity, first.other_entity, encounter_time));
                end_time = encounter_time;
            }
        }
        
        // Solve for any unbounded encounters from the old start time to the new start time
        // Brute force

        time_bounds.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    };
}

#[cfg(test)]
mod test {
    use crate::{debug::get_entity_by_name, systems::trajectory_prediction::test_cases::load_case};

    use super::find_next_encounter;

    #[test]
    fn temp() {
        let (mut state, mut encounters, _, end_time, time_step) = load_case("two-moons-varied-encounters");
        let spacecraft = get_entity_by_name(&state, "spacecraft");
        find_next_encounter(&state, spacecraft, 0.0, 1.0e10)
    }
}