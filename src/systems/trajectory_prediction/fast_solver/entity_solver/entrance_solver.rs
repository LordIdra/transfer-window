use crate::{components::trajectory_component::orbit::Orbit, constants::PREDICTION_TIME_STEP, systems::trajectory_prediction::numerical_methods::itp::itp};

#[cfg(feature = "profiling")]
use tracy_client::span;

/// Solves for a single encounter between the entity and one of its siblings
// pub fn solve_for_entrance(orbit: &Orbit, sibling_orbit: &Orbit, start_time: f64, end_time: f64) -> Option<f64> {
//     let soi = sibling_orbit.get_sphere_of_influence();
//     let f = |time: f64| {
//         let position = orbit.get_position_from_theta(orbit.get_theta_from_time(time));
//         let other_position = sibling_orbit.get_position_from_theta(sibling_orbit.get_theta_from_time(time));
//         (position - other_position).magnitude() - soi
//     };
//     let mut time = start_time;
//     let mut previous_distance = f64::MIN;
//     while time < end_time {
//         let distance = f(time);
//         dbg!(sibling_orbit.get_sphere_of_influence(), distance);
//         if distance.is_sign_negative() && previous_distance.is_sign_positive() {
//             return Some(itp(&f, time, time - PREDICTION_TIME_STEP));
//         }
//         previous_distance = distance;
//         time += PREDICTION_TIME_STEP;
//     }
//     None
// }

pub fn solve_for_entrance_efficient(orbit: &Orbit, sibling_orbit: &Orbit, start_time: f64, end_time: f64) -> Option<f64> {
    let distance = |time: f64| (orbit.get_position_from_theta(orbit.get_theta_from_time(time)) - sibling_orbit.get_position_from_theta(sibling_orbit.get_theta_from_time(time))).magnitude();
    #[cfg(feature = "profiling")]
    let _span = span!("Solve for entrance");
    let distance_prime = |time: f64| (distance(time + 0.001) - distance(time)) / 0.001;
    let distance_prime_start = distance_prime(start_time);
    let distance_prime_end = distance_prime(end_time);
    if distance_prime_start.is_sign_negative() && distance_prime_end.is_sign_positive() {
        let min_distance_time = itp(&distance_prime, start_time, end_time);
        let min_distance = distance(min_distance_time);
        let soi = sibling_orbit.get_sphere_of_influence();
        if min_distance < soi {
            let f = |time: f64| distance(time) - soi;
            return Some(itp(&f, min_distance_time, start_time));
        }
    }
    None
}