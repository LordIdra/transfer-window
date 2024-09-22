use nalgebra_glm::{vec2, DMat2, DVec2};
use transfer_window_common::numerical_methods::closest_ellipse_point::solve_for_closest_point_on_ellipse;

use crate::components::path_component::orbit::Orbit;

/// Returns a function that will return the closest point on the given orbit from an arbitrary point
pub fn make_closest_point_on_ellipse_orbit_function(orbit: &Orbit) -> impl Fn(DVec2) -> DVec2 + '_ {
    assert!(orbit.is_ellipse());
    let a = orbit.semi_major_axis();
    let b = orbit.semi_minor_axis();
    let aop = orbit.argument_of_periapsis();
    let periapsis_position = orbit.position_from_theta(aop);
    let periapsis_to_center_vector = -orbit.semi_major_axis() * vec2(f64::cos(aop), f64::sin(aop));
    let center = periapsis_position + periapsis_to_center_vector;
    let rotate_aop = DMat2::new(aop.cos(), -aop.sin(), aop.sin(), aop.cos());
    let rotate_negative_aop = DMat2::new(aop.cos(), aop.sin(), -aop.sin(), aop.cos());

    move |point: DVec2| {
        let point = rotate_negative_aop * (point - center);
        let point = solve_for_closest_point_on_ellipse(a, b, point);
        rotate_aop * point + center
    }
}

