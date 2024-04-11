use std::{f64::consts::PI, mem::swap};

use nalgebra_glm::DVec2;
use transfer_window_common::numerical_methods::itp::itp;

use crate::components::trajectory_component::orbit::Orbit;

/// Normalizes between 0 and 2pi
/// <https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation>
pub fn normalize_angle(mut theta: f64) -> f64 {
    theta %= 2.0 * PI;
    (theta + 2.0 * PI) % (2.0 * PI)
}

/// Returns the closest point to `point` on the given orbit if it is less than `radius` away from the orbit
/// Returns none if the closest distance to `point` is further than the radius
/// `point` is assumed to be relative to the parent of the orbit
/// Strategy:

/// 2) Construct a quartic representing the intersection between the orbit's conic, and a circle centered on
///    `point` with radius `max_distance`
/// 3) Solve the quartic
/// 4) If there are not exactly two solutions, return none
/// 5) Otherwise, use ITP bounded by the two solutions to find the minimum
pub fn find_closest_point_on_orbit(orbit: &Orbit, point: DVec2, max_distance: f64) -> Option<DVec2> {
    let starting_theta = f64::atan2(point.y, point.x);
    let distance = |time: f64| (orbit.get_position_from_theta(orbit.get_theta_from_time(time)) - point).magnitude();
    let distance_prime = |time: f64| (distance(time + 1.0e-2) - distance(time)) / 1.0e-2;

    let (min_theta, max_theta) = if orbit.is_ellipse() {
        (starting_theta - 0.7, starting_theta + 0.7)
    } else {
        (orbit.get_min_asymptote_theta().unwrap() + 1.0e-2, orbit.get_max_asymptote_theta().unwrap() - 1.0e-2)
    };

    let (mut min_time, mut max_time) = (orbit.get_time_since_last_periapsis(min_theta), orbit.get_time_since_last_periapsis(max_theta));
    let (min, max) = (distance_prime(min_time), distance_prime(max_time));
    if min.is_sign_positive() && max.is_sign_positive() || min.is_sign_negative() && max.is_sign_negative() {
        return None;
    }
    
    if min.is_sign_positive() && max.is_sign_negative() {
        swap(&mut min_time, &mut max_time)
    }

    if orbit.is_ellipse() && min_time > max_time {
        max_time += orbit.get_period().unwrap();
    }

    let time = itp(&distance_prime, min_time, max_time);
    let position = orbit.get_position_from_theta(orbit.get_theta_from_time(time));
    if (position - point).magnitude() < max_distance {
        return Some(position);
    }
    
    None
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use nalgebra_glm::vec2;

    use crate::{components::trajectory_component::orbit::Orbit, storage::entity_allocator::Entity, util::{find_closest_point_on_orbit, normalize_angle}};

    #[test]
    fn test_normalize_angle() {
        assert!((normalize_angle(2.01 * PI) - 0.01 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(3.789 * PI) - 1.789 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(-1.345 * PI) - 0.655 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(-6.0 * PI + 0.2) - 0.2).abs() < 1.0e-5);
    }

    #[test]
    fn test_find_closest_point_on_orbit_ellipse() {
        // Earth orbiting sun
        let orbit = Orbit::new(Entity::mock(), 5.9722e24, 1_988_500e24, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0);

        let c = find_closest_point_on_orbit(&orbit, vec2(1.5e11, 0.0), 1.0e10).unwrap();
        assert!(f64::atan2(c.y, c.x).abs() < 1.0e-2);
        let c = find_closest_point_on_orbit(&orbit, vec2(-1.5e11, -1.0e7), 1.0e10).unwrap();
        assert!((f64::atan2(c.y, c.x) + PI).abs() < 1.0e-2);
        let c = find_closest_point_on_orbit(&orbit, vec2(1.5e11, 0.0), 1.0e7);
        assert!(c.is_none());
        let c = find_closest_point_on_orbit(&orbit, vec2(-1.5e11, 0.0), 1.0e7);
        assert!(c.is_none());
    }

    #[test]
    fn test_find_closest_point_on_orbit_hyperbola() {
        // Hyperbolic moon
        let orbit = Orbit::new(Entity::mock(), 0.07346e24, 5.9722e24, vec2(0.3633e9, 0.0), vec2(0.0, 2.082e3), 0.0);

        let c = find_closest_point_on_orbit(&orbit, vec2(0.36e9, 0.0), 1.0e7).unwrap();
        assert!(f64::atan2(c.y, c.x).abs() < 1.0e-2);
        let c = find_closest_point_on_orbit(&orbit, vec2(1.5e11, 0.0), 1.0e5);
        assert!(c.is_none());
        let c = find_closest_point_on_orbit(&orbit, vec2(-1.5e11, 0.0), 1.0e5);
        assert!(c.is_none());
    }
}