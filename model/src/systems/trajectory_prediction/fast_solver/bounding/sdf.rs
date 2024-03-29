use nalgebra_glm::{vec2, DMat2, DVec2};

#[cfg(feature = "profiling")]
use tracy_client::span;

use crate::components::trajectory_component::orbit::Orbit;

/// <https://stackoverflow.com/a/46007540>
/// <https://blog.chatfield.io/simple-method-for-distance-to-ellipse/>
/// This is a GENIUS very efficient and accurate solution
fn solve_for_closest_point_on_ellipse(a: f64, b: f64, p: DVec2) -> DVec2 {
    // let px = f64::abs(p[0]);
    // let py = f64::abs(p[1]);
    let px = p.x;
    let py = p.y;

    let mut tx = 0.707;
    let mut ty = 0.707;

    for _ in 0..2 {
        let x = a*tx;
        let y = b*ty;

        let ex = (a.powi(2) - b.powi(2)) * tx.powi(3) / a;
        let ey = (b.powi(2) - a.powi(2)) * ty.powi(3) / b;

        let qx = px - ex;
        let qy = py - ey;
        let q = f64::sqrt(qx.powi(2) + qy.powi(2));

        let rx = x - ex;
        let ry = y - ey;
        let r = f64::sqrt(rx.powi(2) + ry.powi(2));

        tx = (qx * r / q + ex) / a;
        ty = (qy * r / q + ey) / b;

        let t = f64::sqrt(ty.powi(2) + tx.powi(2));

        tx /= t;
        ty /= t;
    }

    vec2(a*tx, b*ty)
}


// Returns a function that will return the closest point on the given orbit from an arbitrary point
fn make_closest_point_on_orbit_function(orbit: &Orbit) -> impl Fn(DVec2) -> DVec2 + '_ {
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
    let closest_point_function = make_closest_point_on_orbit_function(orbit_b);
    move |theta: f64| -> f64 {
        #[cfg(feature = "profiling")]
        let _span = span!("SDF");
        let point = orbit_a.get_position_from_theta(theta);
        let other_point = closest_point_function(point);
        let magnitude = (point - other_point).magnitude();
        let sign = (other_point.magnitude_squared() - point.magnitude_squared()).signum();
        sign * magnitude
    }
}
