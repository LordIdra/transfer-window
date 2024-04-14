use crate::{components::trajectory_component::orbit::Orbit, util::make_closest_point_on_ellipse_orbit_function};

/// Returns a function that acts as a signed distance function in terms of an angle on orbit A
/// Negative when orbit_a is OUTSIDE orbit_b
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
