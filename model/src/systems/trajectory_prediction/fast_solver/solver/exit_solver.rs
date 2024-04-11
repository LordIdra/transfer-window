use std::f64::consts::PI;

use transfer_window_common::numerical_methods::itp::itp;

use crate::{components::trajectory_component::orbit::Orbit, Model, storage::entity_allocator::Entity, systems::trajectory_prediction::encounter::{Encounter, EncounterType}};

use super::MIN_TIME_BEFORE_ENCOUNTER;

/// - Create an SDF in terms of theta that's negative outside of the SOI
/// - Solve for the apoapsis (where the SDF will be minimum)
/// - If the SDF at the apoapsis is positive, object will never leave parent SOI
/// - Otherwise, use the ITP solver to find the angle where SDF = 0 (ie, the object leaves the SOI)
/// - The 'side' of the orbit which we use as our bound depends on the orbit direction
/// - Convert the angle to a time, and make sure the time is later than the start time
/// - Finally, check that the time is not after end time - if it is, no encounter is possible
fn find_elliptical_exit_time(orbit: &Orbit, soi: f64, start_time: f64, end_time: f64) -> Option<f64> {
    // SDF negative outside of SOI
    let sdf = |theta: f64| soi - orbit.get_position_from_theta(theta).magnitude(); 
    let periapsis = orbit.get_argument_of_periapsis();
    let apoapsis = periapsis + PI;

    if sdf(apoapsis).is_sign_positive() {
        // Object will never leave the SOI
        return None;
    }
    
    let theta = if orbit.is_clockwise() {
        // Check from periapsis to apoapsis (anticlockwise)
        let mut from = periapsis;
        let to = apoapsis;
        if from < to {
            from += 2.0 * PI;
        }
        itp(&sdf, to, from)

    } else {
        // Check from apoapsis to periapsis (anticlockwise)
        let mut from = apoapsis;
        let to = periapsis;
        if from < to {
            from += 2.0 * PI;
        }
        itp(&sdf, from, to)
    };

    let mut time = orbit.get_first_periapsis_time() + orbit.get_time_since_first_periapsis(theta);
    while time < start_time {
        time += orbit.get_period().unwrap();
    }
    if time > end_time {
        return None;
    }

    Some(time)
}

/// - Start at time = `start_time`
/// - Step time by 1
/// - Calculate position
/// - Check if object has escaped soi; if so:
///   - Use ITP between time and (time - `time_step`) to find exact time of exit and return
///   - Check encounter is before `end_time`
/// - Double the `time_step`
/// - Repeat until t > `end_time`
fn find_hyperbolic_exit_time(orbit: &Orbit, soi: f64, start_time: f64, end_time: f64) -> Option<f64> {
    let f = |time: f64| {
        let theta = orbit.get_theta_from_time(time);
        let distance = orbit.get_position_from_theta(theta).magnitude();
        soi - distance
    };

    let mut previous_f = f(start_time);
    let mut time_step = 4.0;
    while start_time + time_step < end_time {
        // Time must be incremented so we overshoot the end time
        // This is because there could be an encounter between the previous time (before end time)
        // and the new time (after end time)
        time_step *= 2.0;
        let time = start_time + time_step;
        let new_f = f(time);
        if new_f.is_sign_negative() && previous_f.is_sign_positive() {
            let min = time - (time_step / 2.0);
            let max = time;
            let encounter_time = itp(&f, max, min);
            if encounter_time > end_time {
                return None;
            }
            return Some(encounter_time);
        }
        previous_f = new_f;
    }
    None
}

/// Solves for when an entity will leave its parent
/// If the given entity is not on a hyperbolic trajectory, returns none when a call to solve is made
pub fn solve_for_exit(model: &Model, entity: Entity, start_time: f64, end_time: f64) -> Option<Encounter> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Solve for exit");
    let orbit = model.get_trajectory_component(entity).get_end_segment().as_orbit(); 
    let Some(parent_trajectory_component) = model.try_get_trajectory_component(orbit.get_parent()) else {
        // Parent cannot be exited as it is a root entity
        return None;
    };
    let parent_orbit = parent_trajectory_component.get_segment_at_time(start_time).as_orbit();

    let encounter_time = if orbit.is_ellipse() {
        find_elliptical_exit_time(orbit, parent_orbit.get_sphere_of_influence(), start_time, end_time)
    } else {
        find_hyperbolic_exit_time(orbit, parent_orbit.get_sphere_of_influence(), start_time, end_time)
    }?;
    
    if encounter_time < start_time + MIN_TIME_BEFORE_ENCOUNTER {
        // Another encounter could be calculated as being eg 0.01 seconds later
        // if eg an entity exits an SOI and then an 'entrance' is calculated to be very shortly after
        // So we add MIN_TIME_BEFORE_ENCOUNTER
        return None;
    }

    Some(Encounter::new(EncounterType::Exit, entity, parent_orbit.get_parent(), encounter_time))
}