use nalgebra_glm::{vec2, DMat2, DVec2};
use transfer_window_common::numerical_methods::closest_ellipse_point::solve_for_closest_point_on_ellipse;

use crate::components::trajectory_component::orbit::Orbit;

// Returns a function that will return the closest point on the given orbit from an arbitrary point
pub fn make_closest_point_on_ellipse_orbit_function(orbit: &Orbit) -> impl Fn(DVec2) -> DVec2 + '_ {
    assert!(orbit.is_ellipse());
    let a = orbit.get_semi_major_axis();
    let b = orbit.get_semi_minor_axis();
    let aop = orbit.get_argument_of_periapsis();
    let periapsis_position = orbit.get_position_from_theta(aop);
    let periapsis_to_center_vector = -orbit.get_semi_major_axis() * vec2(f64::cos(aop), f64::sin(aop));
    let center = periapsis_position + periapsis_to_center_vector;
    let rotate_aop = DMat2::new(aop.cos(), -aop.sin(), aop.sin(), aop.cos());
    let rotate_negative_aop = DMat2::new(aop.cos(), aop.sin(), -aop.sin(), aop.cos());

    move |point: DVec2| {
        let point = rotate_negative_aop * (point - center);
        let point = solve_for_closest_point_on_ellipse(a, b, point);
        rotate_aop * point + center
    }
}

// Returns a function that acts as a signed distance function in terms of an angle on orbit A
// Negative when orbit_a is OUTSIDE orbit_b
pub fn make_sdf<'a>(orbit_a: &'a Orbit, orbit_b: &'a Orbit) -> impl Fn(f64) -> f64 + 'a  {
    let closest_point_function = make_closest_point_on_ellipse_orbit_function(orbit_b);
    move |theta: f64| -> f64 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("SDF");
        let point = orbit_a.get_position_from_theta(theta);
        let other_point = closest_point_function(point);
        let magnitude = (point - other_point).magnitude();
        let sign = (other_point.magnitude_squared() - point.magnitude_squared()).signum();
        sign * magnitude
    }
}
