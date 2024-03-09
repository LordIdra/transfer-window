use crate::{components::trajectory_component::orbit::Orbit, constants::MIN_TIME_BEFORE_ENCOUNTER, state::State, storage::entity_allocator::Entity, systems::trajectory_prediction::{numerical_methods::bisection::bisection, util::{Encounter, EncounterType}}};

/// Strategy: 
/// - Start at time = start_time
/// - Step time by 1
/// - Calculate position
/// - Check if object has escaped soi; if so:
///   - Use bisection between time and (time - time_step) to find exact time of exit and return
///   - Check encounter is before end_time
/// - Double the time step
/// - Repeat until t > end_time
fn find_exit_time(orbit: &Orbit, soi: f64, start_time: f64, end_time: f64) -> Option<f64> {
    let f = |time: f64| {
        let theta = orbit.get_theta_from_time(time);
        let distance = orbit.get_position_from_theta(theta).magnitude();
        soi - distance
    };

    let mut time = start_time;
    let mut time_step = 4.0;
    while time < end_time {
        // Time must be incremented so we overshoot the end time
        // This is because there could be an encounter between the previous time (before end time)
        // and the new time (after end time)
        time_step *= 2.0;
        time += time_step;
        if f(time) < 0.0 {
            let min = time - time_step;
            let max = time;
            println!("bisecting exit {} to {}", min, max);
            let encounter_time = bisection(&f, min, max, 24);
            if encounter_time > end_time {
                return None;
            }
            return Some(encounter_time);
        }
    }
    None
}

/// Solves for when an entity will leave its parent
/// If the given entity is not on a hyperbolic trajectory, returns none when a call to solve is made
pub fn solve_for_exit(state: &State, entity: Entity, start_time: f64, end_time: f64) -> Option<Encounter> {
    let orbit = state.get_trajectory_component(entity).get_end_segment().as_orbit(); 
    if orbit.is_ellipse() {
        // TODO Orbit is an ellipse
        return None;
    }
    let Some(parent_trajectory_component) = state.try_get_trajectory_component(orbit.get_parent()) else {
        // Parent cannot be exited as it is a root entity
        return None;
    };
    let parent_orbit = parent_trajectory_component.get_segment_at_time(start_time).as_orbit();
    let Some(encounter_time) = find_exit_time(orbit, parent_orbit.get_sphere_of_influence(), start_time, end_time) else {
        return None;
    };
    if encounter_time < start_time + MIN_TIME_BEFORE_ENCOUNTER {
        // Another encounter could be calculated as being eg 0.01 seconds later
        // if eg an entity exits an SOI and then an 'entrance' is calculated to be very shortly after
        return None;
    }
    return Some(Encounter::new(EncounterType::Exit, entity, parent_orbit.get_parent(), encounter_time))
}