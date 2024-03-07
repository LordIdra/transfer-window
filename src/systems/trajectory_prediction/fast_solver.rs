use std::cmp::Ordering;

use crate::{components::{trajectory_component::orbit::Orbit, ComponentType}, constants::PREDICTION_TIME_STEP, state::State, storage::entity_allocator::Entity, systems::trajectory_prediction::{numerical_methods::bisection::bisection, util::get_parallel_entities}};

use self::bounding::{find_encounter_bounds, EncounterBoundType};

use super::util::{Encounter, EncounterType};

mod bounding;

fn angle_window_to_time_window(orbit: &Orbit, window: (f64, f64)) -> (f64, f64) {
    (orbit.get_first_periapsis_time() + orbit.get_time_since_first_periapsis(window.0), orbit.get_first_periapsis_time() + orbit.get_time_since_first_periapsis(window.1))
}

fn solve_for_encounter_ellipse_ellipse(orbit: &Orbit, other_orbit: &Orbit, start_time: f64, end_time: f64) -> Option<f64> {
    let f = |time: f64| {
        let position = orbit.get_position_from_theta(orbit.get_theta_from_time(time));
        let other_position = other_orbit.get_position_from_theta(other_orbit.get_theta_from_time(time));
        (position - other_position).magnitude()
    };
    let soi = other_orbit.get_sphere_of_influence();
    let mut time = start_time;
    let mut previous_distance = f64::MAX;
    while time < end_time {
        let distance = f(time) - soi;
        if distance.is_sign_negative() && previous_distance.is_sign_positive() {
            return Some(bisection(&f, time - PREDICTION_TIME_STEP, time));
        }
        previous_distance = distance;
        time += PREDICTION_TIME_STEP;
    }
    None
}

fn solve_for_encounter(orbit: &Orbit, other_orbit: &Orbit, start_time: f64, end_time: f64) -> Option<f64> {
    if orbit.get_eccentricity().is_sign_positive() && orbit.get_eccentricity().is_sign_positive() {
        solve_for_encounter_ellipse_ellipse(orbit, other_orbit, start_time, end_time)
    } else if orbit.get_eccentricity().is_sign_negative() {
        // A hyperbola B ellipse
    } else if other_orbit.get_eccentricity().is_sign_negative() {
        // A ellipse B hyperbola
    } else {
        // both hyperbola
    }
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

struct Unbounded<'a> {
    orbit: &'a Orbit,
    other_orbit: &'a Orbit,
    other_entity: Entity,
}

impl<'a> Unbounded<'a> {
    pub fn new(orbit: &'a Orbit, other_orbit: &'a Orbit, other_entity: Entity) -> Self {
        Self { orbit, other_orbit, other_entity }
    }
}

fn get_initial_bounds(state: &State, entity: Entity, start_time: f64) -> (Vec<Unbounded>, Vec<TimeWindow>) {
    let orbit = state.get_trajectory_component(entity).get_end_segment().as_orbit();
    let can_enter = &state.get_entities(vec![ComponentType::TrajectoryComponent, ComponentType::OrbitableComponent]);
    let parallel_entities = get_parallel_entities(state, can_enter, entity);
    let mut unbounded = vec![];
    let mut time_bounds = vec![];
    for other_entity in parallel_entities {
        let other_orbit = state.get_trajectory_component(other_entity).get_segment_at_time(start_time).as_orbit();
        let bounds = find_encounter_bounds(orbit, other_orbit);
        match bounds {
            EncounterBoundType::NoEncounters => (),
            EncounterBoundType::NoBounds => unbounded.push(Unbounded::new(orbit, other_orbit, other_entity)),
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

fn should_update(soonest_encounter: &Option<Encounter>, encounter_time: f64) -> bool {
    if let Some(soonest_encounter) = soonest_encounter {
        encounter_time < soonest_encounter.get_time()
    } else {
        true
    }
}

fn update_soonest_encounter(soonest_encounter: &mut Option<Encounter>, orbit: &Orbit, other_orbit: &Orbit, entity: Entity, other_entity: Entity, start_time: f64, end_time: &mut f64) {
    if let Some(encounter_time) = solve_for_encounter(orbit, other_orbit, start_time, *end_time) {
        if encounter_time > 0.0 && should_update(&soonest_encounter, encounter_time) {
            *soonest_encounter = Some(Encounter::new(EncounterType::Entrance, entity, other_entity, encounter_time));
            *end_time = encounter_time;
        }
    }
}

pub fn find_next_encounter(state: &State, entity: Entity, start_time: f64, end_time: f64) -> Option<Encounter> {
    let (unbounded, mut time_bounds) = get_initial_bounds(state, entity, start_time);
    let mut start_time = if let Some(first) = time_bounds.first() {
        first.get_soonest_time()
    } else {
        start_time
    };
    let mut end_time = end_time;

    if time_bounds.is_empty() {
        let mut soonest_encounter: Option<Encounter> = None;
        for unbounded in unbounded {
            update_soonest_encounter(&mut soonest_encounter, unbounded.orbit, unbounded.other_orbit, entity, unbounded.other_entity, start_time, &mut end_time);
        }
        return soonest_encounter
    }

    let mut soonest_encounter: Option<Encounter> = None;
    loop {
        if start_time > end_time {
            break;
        }

        let first = time_bounds.first_mut().unwrap();
        let new_start_time = first.get_soonest_time();
        update_soonest_encounter(&mut soonest_encounter, first.orbit, first.other_orbit, entity, first.other_entity, start_time, &mut end_time);
        first.increment_by_period();
        for unbounded in &unbounded {
            update_soonest_encounter(&mut soonest_encounter, unbounded.orbit, unbounded.other_orbit, entity, unbounded.other_entity, start_time, &mut end_time);
        }
        
        start_time = new_start_time;
        time_bounds.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    };

    return soonest_encounter;
}

#[cfg(test)]
mod test {
    use crate::{components::ComponentType, debug::get_entity_by_name, systems::trajectory_prediction::{test_cases::load_case, util::{apply_encounter, Encounter}}};

    use super::find_next_encounter;

    #[test]
    fn temp() {
        let (mut state, mut encounters, _, end_time, time_step) = load_case("insanity-1");
        let spacecraft = get_entity_by_name(&state, "spacecraft");
        println!("{:?}", find_next_encounter(&state, spacecraft, 0.0, 1.0e10));
    }

    fn run_case(name: &str) {
        let (mut state, mut encounters, _, end_time, time_step) = load_case(name);

        let mut start_time = 0.0;
        loop {
            let mut soonest_encounter: Option<Encounter> = None;
            for entity in state.get_entities(vec![ComponentType::TrajectoryComponent]) {
                println!("{}", state.get_name_component(entity).get_name());
                let encounter = find_next_encounter(&state, entity, start_time, end_time);
                if let Some(encounter) = encounter {
                    if let Some(soonest_encounter) = &mut soonest_encounter {
                        if encounter.get_time() <  soonest_encounter.get_time() {
                            *soonest_encounter = encounter;
                        }
                    } else {
                        soonest_encounter = Some(encounter);
                    }
                }
            }

            if soonest_encounter.is_none() {
                break;
            }

            
            let encounter = soonest_encounter.unwrap();
            
            for entity in state.get_entities(vec![ComponentType::TrajectoryComponent]) {
                state.get_trajectory_component_mut(entity).get_end_segment_mut().as_orbit_mut().end_at(encounter.get_time());
            }
            
            println!("{:?}", encounter);

            let Some(next_encounter) = encounters.front() else {
                panic!("Found unexpected encounter: {:#?}", encounter);
            };
            if !next_encounter.compare(&state, &encounter) {
                panic!("Encounters not equal: {:#?} {:#?}", encounter, encounters.front())
            }
            encounters.pop_front();
            start_time = encounter.get_time();
            apply_encounter(&mut state, encounter);
        }

        if !encounters.is_empty() {
            panic!("Missed encounters: {:?}", encounters);
        }
    }

    #[test]
    fn test_case_collision_with_moon() {
        run_case("collision-with-moon");
    }

    #[test]
    fn test_case_encounter_with_earth() {
        run_case("encounter-with-earth");
    }

    #[test]
    fn test_case_encounter_with_earth_and_moon() {
        run_case("encounter-with-earth-and-moon");
    }

    #[test]
    fn test_case_escape_from_earth() {
        run_case("escape-from-earth");
    }

    #[test]
    fn test_case_insanity_1() {
        run_case("insanity-1");
    }

    #[test]
    fn test_case_insanity_2() {
        run_case("insanity-2");
    }

    #[test]
    fn test_case_insanity_3() {
        run_case("insanity-3");
    }

    #[test]
    fn test_case_many_moon_encounters() {
        run_case("many-moon-encounters");
    }

    #[test]
    fn test_case_moon_slingshot_to_escape_earth() {
        run_case("moon-slingshot-to-escape-earth");
    }

    #[test]
    fn test_case_no_encounters() {
        run_case("no-encounters");
    }

    #[test]
    fn test_case_two_moons_varied_encounter() {
        run_case("two-moons-varied-encounters");
    }
}