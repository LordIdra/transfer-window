use std::f64::consts::PI;

use nalgebra_glm::DVec2;

#[cfg(feature = "profiling")]
use tracy_client::span;

use crate::{components::trajectory_component::orbit::Orbit, systems::trajectory_prediction::numerical_methods::{bisection::bisection, laguerre::laguerre_to_find_stationary_point}, util::normalize_angle};

// Assuming we've already found a stationary point on a periodic function with 1 minimum and 1 maximum,
// we can find the other using bisection by just about excluding the known stationary point
fn find_other_stationary_point(known_stationary_point_theta: f64, distance_function: impl Fn(f64) -> f64) -> f64 {
    let min = known_stationary_point_theta + 0.001;
    let max = known_stationary_point_theta - 0.001 + 2.0*PI;
    let derivative = |theta: f64| (distance_function(theta + 0.00001) - distance_function(theta)) / 0.00001;
    let starting_estimate = bisection(&derivative, min, max, 1.0e-2, 64);
    laguerre_to_find_stationary_point(&distance_function, starting_estimate, 1.0e-6, 16)
}

// Returns a function that will return the closest point on the given orbit from an arbitrary point
fn make_closest_point_on_orbit_function(orbit: &Orbit) -> impl Fn(DVec2) -> DVec2 + '_ {
    move |point: DVec2| {
        #[cfg(feature = "profiling")]
        let _span = span!("Closest point on orbit");
        let distance_function = |theta: f64| (orbit.get_position_from_theta(theta) - point).magnitude();
        let starting_theta = f64::atan2(point.y, point.x);
        let mut theta = laguerre_to_find_stationary_point(&distance_function, starting_theta + 0.1, 1.0e-6, 256);
        // weird bug occurred here where sometimes the distance function would be less slightly after if the value of theta was too low
        // this is why we add slightly more to theta and check both before and after
        if distance_function(theta + 0.001) < distance_function(theta) && distance_function(theta - 0.001) < distance_function(theta) {
            // we found a maximum, but want a minimum
            theta = find_other_stationary_point(theta, distance_function);
        }
        orbit.get_position_from_theta(theta)

    }
}

// Returns a function that acts as a signed distance function in terms of an angle on orbit A
pub fn make_sdf<'a>(orbit_a: &'a Orbit, orbit_b: &'a Orbit) -> impl Fn(f64) -> f64 + 'a  {
    let closest_point_function = make_closest_point_on_orbit_function(orbit_b);
    move |theta: f64| -> f64 {
        #[cfg(feature = "profiling")]
        let _span = span!("SDF");
        let point = orbit_a.get_position_from_theta(theta);
        let other_point = closest_point_function(point);
        let magnitude = (point - other_point).magnitude();
        let sign = (other_point.magnitude() - point.magnitude()).signum();
        sign * magnitude
    }
}

pub fn find_min_max_signed_distance(sdf: &impl Fn(f64) -> f64, argument_of_apoapsis: f64) -> (f64, f64) {
    #[cfg(feature = "profiling")]
    let _span = span!("Min max signed distance");
    let theta_1 = laguerre_to_find_stationary_point(&sdf, argument_of_apoapsis, 1.0e-6, 256);
    let theta_2 = find_other_stationary_point(theta_1, &sdf);
    let (theta_1, theta_2) = (normalize_angle(theta_1), normalize_angle(theta_2));
    if sdf(theta_1) < sdf(theta_2) { 
        (theta_1, theta_2)
    } else { 
        (theta_2, theta_1)
    }
}