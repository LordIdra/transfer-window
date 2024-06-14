use crate::components::path_component::orbit::Orbit;

use transfer_window_common::numerical_methods::itp::itp;

/// There exists an incredibly powerful technique to find encounters between two times:
/// - We first evaluate the derivatives of distance D between the opbjects with respect to time t (dD/dt) on both sides of the window
/// - If dD/dt < 0 on the start side and dD/dt > 0 on the end side, there must be a minimum between them
/// - The minimum is bracketed, so we use the ITP solver to find the time of the minimum
/// - If D at the time of the minimum is less than the SOI, we know a encounter occurs between the start time and the time of the minimum
/// 
/// We need to find this minimum:
/// - We create a function f(t) = D(t) - soi which is negative at minimum and positive at the start
/// - When f(t) = 0, an encounter occurs, and since the encounter is bracketed, we use ITP to find the encounter time
/// 
/// Here is the catch: When solving for the minimum, the start time of the bound may not actually be positive
/// The solution to this is to keep moving it back by a fixed amount until it's positive, then deploy ITP
pub fn solve_for_entrance(orbit: &Orbit, sibling_orbit: &Orbit, start_time: f64, end_time: f64) -> Result<Option<f64>, &'static str> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Solve for entrance");
    let distance = |time: f64| (orbit.position_from_theta(orbit.theta_from_time(time)) - sibling_orbit.position_from_theta(sibling_orbit.theta_from_time(time))).magnitude();
    let distance_prime = |time: f64| (distance(time + 0.001) - distance(time)) / 0.001;
    let distance_prime_start = distance_prime(start_time);
    let distance_prime_end = distance_prime(end_time);
    if distance_prime_start.is_sign_negative() && distance_prime_end.is_sign_positive() {
        let min_distance_time = itp(&distance_prime, start_time, end_time)?;
        let min_distance = distance(min_distance_time);
        let soi = sibling_orbit.sphere_of_influence();
        if min_distance < soi {
            let f = |time: f64| distance(time) - soi;
            let mut adjusted_start_time = start_time;
            if let Some(period) = orbit.period() {
                while f(adjusted_start_time).is_sign_negative() {
                    adjusted_start_time -= period / 32.0;
                }
            }
            return Ok(Some(itp(&f, min_distance_time, adjusted_start_time)?));
        }
    }
    Ok(None)
}