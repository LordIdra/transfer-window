use std::{f64::consts::PI, mem::swap};

use crate::{components::trajectory_component::orbit::Orbit, systems::trajectory_prediction::numerical_methods::itp::itp, util::normalize_angle};

#[cfg(feature = "profiling")]
use tracy_client::span;

// Constructs a range with theta 1 and theta 2 containing 'containing'
// This is harder than it first appears, because for example the range 5.9 to 5.8 contains the angle 1.4
// We can work around this by considering both cases, 5.9 to 5.8 (out of order) and 5.8 to 5.9 (in order)
// Then check if the in order case contains the minimum
// If so, that's our solution. If not, the other case is the solution
pub fn make_range_containing(theta_1: f64, theta_2: f64, containing: f64) -> (f64, f64) {
    let theta_1 = normalize_angle(theta_1);
    let theta_2 = normalize_angle(theta_2);
    let containing = normalize_angle(containing);
    let in_order = (f64::min(theta_1, theta_2), f64::max(theta_1, theta_2));
    let out_of_order = (in_order.1, in_order.0);
    if containing > in_order.0 && containing < in_order.1 {
        in_order
    } else {
        out_of_order
    }
}

// Returns the (smallest) distance from the first angle to the second, ie wrapping round if necessary
pub fn angular_distance(from: f64, to: f64) -> f64 {
    let from = normalize_angle(from);
    let to = normalize_angle(to);
    if from < to {
        to - from
    } else {
        to + 2.0*PI - from
    }
}

pub fn angle_window_to_time_window(orbit: &Orbit, mut window: (f64, f64)) -> (f64, f64) {
    if orbit.is_clockwise() {
        swap(&mut window.0, &mut window.1);
    }
    let mut window = (
        orbit.get_first_periapsis_time() + orbit.get_time_since_first_periapsis(window.0), 
        orbit.get_first_periapsis_time() + orbit.get_time_since_first_periapsis(window.1));
    if window.1 < window.0 {
        window.1 += orbit.get_period().unwrap()
    }
    window
}

// Assuming we've already found a stationary point on a periodic function with 1 minimum and 1 maximum,
// we can find the other by creating a range that just about excludes the known stationary point
// Should only be used on ellipse SDFs
pub fn find_other_stationary_point(distance_function: impl Fn(f64) -> f64, known_stationary_point_theta: f64) -> f64 {
    #[cfg(feature = "profiling")]
    let _span = span!("Find other stationary point");
    let mut min = known_stationary_point_theta - 0.001;
    let mut max = known_stationary_point_theta + 0.001 - 2.0*PI;
    let derivative = |theta: f64| (distance_function(theta + 0.0001) - distance_function(theta)) / 0.0001;
    if derivative(min).is_sign_positive() {
        swap(&mut min, &mut max);
    }
    itp(&derivative, min, max)
}


#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::systems::trajectory_prediction::fast_solver::bounding::util::make_range_containing;

    fn compare_f64_pairs(pair_1: (f64, f64), pair_2: (f64, f64)) -> bool {
        (pair_1.0 - pair_2.0).abs() < 1.0e-6 && (pair_1.1 - pair_2.1).abs() < 1.0e-6
    }

    #[test]
    fn test_make_range_containing() {
        assert!(make_range_containing(0.0, 3.0, 2.0) == (0.0, 3.0));
        assert!(make_range_containing(0.0, 3.0, 5.0) == (3.0, 0.0));
        assert!(make_range_containing(-2.0, 2.0, 0.1) == (-2.0 + 2.0*PI, 2.0));
        assert!(make_range_containing(-2.0, 2.0, 2.8) == (2.0, -2.0 + 2.0*PI));
    }
}