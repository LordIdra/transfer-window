use std::f64::consts::PI;

use nalgebra_glm::DVec2;

use crate::{components::trajectory_component::orbit::Orbit, systems::trajectory_prediction::numerical_methods::{bisection::bisection, newton_raphson::{newton_raphson, newton_raphson_to_find_stationary_point}}, util::normalize_angle};

// The contained slices are windows in terms of theta of where an encounter could happen
// They are ordered, ie (0.5, 2.4) means an encounter could happen between 0.5 and 2.4, not the other way round
#[derive(Debug)]
pub enum EncounterBounds {
    None,
    One((f64, f64)),
    Two((f64, f64), (f64, f64)),
}

// Assuming we've already found a stationary point on a periodic function with 1 minimum and 1 maximum,
// we can find the other using bisection by just about excluding the known stationary point
fn find_other_stationary_point(known_stationary_point_theta: f64, distance_function: impl Fn(f64) -> f64) -> f64 {
    let min = known_stationary_point_theta + 0.001;
    let max = known_stationary_point_theta - 0.001 + 2.0*PI;
    let derivative = move |theta: f64| (distance_function(theta + 0.00001) - distance_function(theta)) / 0.00001;
    bisection(&derivative, min, max)
}

// Returns a function that will return the closest point on the given orbit from an arbitrary point
fn make_closest_point_on_orbit_function(orbit: &Orbit) -> impl Fn(DVec2) -> DVec2 + '_ {
    move |point: DVec2| {
        let distance_function = |theta: f64| (orbit.get_position_from_theta(theta) - point).magnitude();
        let starting_theta = f64::atan2(point.y, point.x);
        let mut theta = newton_raphson_to_find_stationary_point(&distance_function, starting_theta)
            .expect("Newton-Raphson failed to converge to stationary point when finding encounter bound");
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
fn make_sdf<'a>(orbit_a: &'a Orbit, orbit_b: &'a Orbit) -> impl Fn(f64) -> f64 + 'a  {
    let closest_point_function = make_closest_point_on_orbit_function(orbit_b);
    move |theta: f64| -> f64 {
        let point = orbit_a.get_position_from_theta(theta);
        let other_point = closest_point_function(point);
        let magnitude = (point - other_point).magnitude();
        let sign = (other_point.magnitude() - point.magnitude()).signum();
        sign * magnitude
    }
}

fn find_min_max_signed_distance(sdf: &impl Fn(f64) -> f64, argument_of_apoapsis: f64) -> (f64, f64) {
    let theta_1 = newton_raphson_to_find_stationary_point(&sdf, argument_of_apoapsis)
        .expect("Newton-Raphson failed to converge to stationary point when finding encounter bound");
    let theta_2 = find_other_stationary_point(theta_1, &sdf);
    let (theta_1, theta_2) = (normalize_angle(theta_1), normalize_angle(theta_2));
    if sdf(theta_1) < sdf(theta_2) { 
        (theta_1, theta_2)
    } else { 
        (theta_2, theta_1)
    }
}

// Constructs a range with theta 1 and theta 2 containing 'containing'
// This is harder than it first appears, because for example the range 5.9 to 5.8 contains the angle 1.4
// We can work around this by considering both cases, 5.9 to 5.8 (out of order) and 5.8 to 5.9 (in order)
// Then check if the in order case contains the minimum
// If so, that's our solution. If not, the other case is the solution
fn make_range_containing(theta_1: f64, theta_2: f64, containing: f64) -> (f64, f64) {
    let theta_1 = normalize_angle(theta_1);
    let theta_2 = normalize_angle(theta_2);
    let in_order = (f64::min(theta_1, theta_2), f64::max(theta_1, theta_2));
    let out_of_order = (in_order.1, in_order.0);
    if containing > in_order.0 && containing < in_order.1 {
        in_order
    } else {
        out_of_order
    }
}

// Returns the (smallest) distance from the first angle to the second, ie wrapping round if necessary
fn angular_distance(from: f64, to: f64) -> f64 {
    let from = normalize_angle(from);
    let to = normalize_angle(to);
    if from < to {
        to - from
    } else {
        to + 2.0*PI - from
    }
}

fn find_smallest_window_containing(containing: f64, thetas: &Vec<f64>) -> (f64, f64) {
    let mut min_distance = f64::MAX;
    let mut smallest_window = None;
    for theta_1 in thetas {
        for theta_2 in thetas {
            if theta_1 == theta_2 {
                continue;
            }
            let (min, max) = make_range_containing(*theta_1, *theta_2, containing);
            if angular_distance(min, max) < min_distance {
                smallest_window = Some((min, max));
                min_distance = angular_distance(min, max);
            }
        }
    }
    smallest_window.unwrap()
}

fn find_intersections(f: &impl Fn(f64) -> f64, min_theta: f64, max_theta: f64) -> (f64, f64) {
    let theta_1 = normalize_angle(newton_raphson(&f, bisection(&f, min_theta, max_theta)).expect("Newton-Raphson failed to converge"));
    // the other angle is in the 'opposite' range
    // we can find this by subtracting 2pi from the highest theta
    let (new_min_theta, new_max_theta) = if min_theta > max_theta {
        (min_theta - 2.0 * PI, max_theta)
    } else {
        (min_theta, max_theta - 2.0 * PI)
    };
    let theta_2 = normalize_angle(newton_raphson(&f, bisection(&f, new_min_theta, new_max_theta)).expect("Newton-Raphson failed to converge"));
    (theta_1, theta_2)
}

// Object A: The object which will have encounters
// Object B: The object that object A will encounter
pub fn find_encounter_bounds(orbit_a: &Orbit, orbit_b: &Orbit) -> EncounterBounds {
    // we start at the angle of apoapsis (rather than periapsis) because this is where the distance is most sensitive to the angle
    // so the starting angle there is actually more likely to be closer to the real solution
    let argument_of_apoapsis = orbit_a.get_argument_of_periapsis() + PI;
    let sdf = make_sdf(orbit_a, orbit_b);
    let (min_theta, max_theta) = find_min_max_signed_distance(&sdf, argument_of_apoapsis);
    let (min, max) = (sdf(min_theta), sdf(max_theta));
    let soi = orbit_b.get_sphere_of_influence();
    if min.is_sign_positive() && min > soi {
        EncounterBounds::None
    } else if min.is_sign_negative() && min.abs() > soi {
        let f_inner = |theta: f64| sdf(theta) - soi;
        let f_outer = |theta: f64| sdf(theta) + soi;
        let inner_intersections = find_intersections(&f_inner, min_theta, max_theta);
        let outer_intersections = find_intersections(&f_outer, min_theta, max_theta);
        let zero_intersections = find_intersections(&sdf, min_theta, max_theta);
        let all_intersections = vec![inner_intersections.0, inner_intersections.1, outer_intersections.0, outer_intersections.1];
        EncounterBounds::Two(find_smallest_window_containing(zero_intersections.0, &all_intersections), find_smallest_window_containing(zero_intersections.1, &all_intersections))
    } else {
        let f = |theta: f64| sdf(theta) - soi;
        let intersections = find_intersections(&f, min_theta, max_theta);
        let window = make_range_containing(intersections.0, intersections.1, min_theta);
        EncounterBounds::One(window)
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::systems::trajectory_prediction::fast_solver::bounding::{find_smallest_window_containing, make_range_containing};

    #[test]
    fn test_make_range_containing() {
        assert!(make_range_containing(0.0, 3.0, 2.0) == (0.0, 3.0));
        assert!(make_range_containing(0.0, 3.0, 5.0) == (3.0, 0.0));
        assert!(make_range_containing(-2.0, 2.0, 0.1) == (-2.0 + 2.0*PI, 2.0));
        assert!(make_range_containing(-2.0, 2.0, 2.8) == (2.0, -2.0 + 2.0*PI));
    }

    #[test]
    fn test_find_smallest_window_containing() {
        dbg!(find_smallest_window_containing(0.0, &vec![0.1, 4.0, 3.2, -0.5, -0.25]));
        assert!(find_smallest_window_containing(0.0, &vec![0.1, 4.0, 3.2, -0.5, -0.25]) == (-0.25 + 2.0*PI, 0.1));
        assert!(find_smallest_window_containing(3.9, &vec![0.1, 4.0, 3.2, -0.5, -0.25]) == (3.2, 4.0));
        assert!(find_smallest_window_containing(-0.3, &vec![0.1, 4.0, 3.2, -0.5, -0.25]) == (-0.5 + 2.0*PI, -0.25 + 2.0*PI));
    }
}