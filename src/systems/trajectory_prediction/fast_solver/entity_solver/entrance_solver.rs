use crate::{components::trajectory_component::orbit::Orbit, constants::PREDICTION_TIME_STEP, systems::trajectory_prediction::numerical_methods::bisection::bisection};

fn solve_ellipse_ellipse(orbit: &Orbit, sibling_orbit: &Orbit, start_time: f64, end_time: f64) -> Option<f64> {
    let f = |time: f64| {
        let position = orbit.get_position_from_theta(orbit.get_theta_from_time(time));
        let other_position = sibling_orbit.get_position_from_theta(sibling_orbit.get_theta_from_time(time));
        (position - other_position).magnitude()
    };
    let soi = sibling_orbit.get_sphere_of_influence();
    let mut time = start_time;
    let mut previous_distance = f64::MAX;
    while time < end_time {
        let distance = f(time) - soi;
        if distance.is_sign_negative() && previous_distance.is_sign_positive() {
            return Some(bisection(&f, time - PREDICTION_TIME_STEP, time));
        }
        previous_distance = distance;
        time += PREDICTION_TIME_STEP;
    }
    None
}

/// Solves for a single encounter between the entity and one of its siblings
pub fn solve_for_entrance(orbit: &Orbit, sibling_orbit: &Orbit, start_time: f64, end_time: f64) -> Option<f64> {
    if orbit.get_semi_major_axis().is_sign_positive() && orbit.get_semi_major_axis().is_sign_positive() {
        // Both ellipse
        solve_ellipse_ellipse(orbit, sibling_orbit, start_time, end_time)

    } else if orbit.get_semi_major_axis().is_sign_negative() {
        // A hyperbola B ellipse
        None

    } else if sibling_orbit.get_semi_major_axis().is_sign_negative() {
        // A ellipse B hyperbola
        None
        
    } else {
        // Both hyperbola
        None
    }
}