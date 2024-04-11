use std::f64::consts::PI;
use eframe::emath::normalized_angle;
use nalgebra_glm::DVec2;
use transfer_window_model::components::trajectory_component::orbit::Orbit;

use super::util::tessellate;

const INITIAL_POINT_COUNT: usize = 30;

fn find_initial_points_orbit(orbit: &Orbit, absolute_parent_position: DVec2) -> Vec<DVec2> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Find initial points");
    let start_angle = normalized_angle(orbit.get_current_point().get_theta() as f32);
    let angle_to_rotate_through = orbit.get_remaining_angle();

    // First and last points are be equal to make sure we draw lines through the whole segment
    let mut initial_points = vec![];
    for i in 0..=INITIAL_POINT_COUNT {
        let theta = start_angle as f64 + (i as f64 / INITIAL_POINT_COUNT as f64) * angle_to_rotate_through;
        initial_points.push(absolute_parent_position + orbit.get_position_from_theta(theta));
    }

    initial_points
}

/// Assumes the angles are within 2pi of each other
fn interpolate_angles(mut angle_1: f64, mut angle_2: f64) -> f64 {
    // This is confusing, but helps prevent edge cases where eg 6.0 and 0.1 have an 'average' of 3.05
    if (angle_1 - angle_2).abs() > PI {
        if angle_1 < angle_2 {
            angle_1 += 2.0 * PI;
        } else {
            angle_2 += 2.0 * PI;
        }
    }
    (angle_1 + angle_2) / 2.0
}

pub fn compute_points(orbit: &Orbit, absolute_parent_position: DVec2, camera_centre: DVec2, zoom: f64) -> Vec<DVec2> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Compute points");
    let points = find_initial_points_orbit(orbit, absolute_parent_position);
    let interpolate = |point1: DVec2, point2: DVec2| {
        let theta_1 = f64::atan2(point1.y, point1.x);
        let theta_2 = f64::atan2(point2.y, point2.x);
        orbit.get_position_from_theta(interpolate_angles(theta_1, theta_2))
    };
    tessellate(interpolate, points, absolute_parent_position, camera_centre, zoom)
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::game::underlay::segments::orbit::interpolate_angles;

    #[test]
    fn test_interpolate_angles() {
        assert!((interpolate_angles(0.1, 0.3) - 0.2).abs() < 1.0e-3);
        assert!((interpolate_angles(-0.1, 0.3) - 0.1).abs() < 1.0e-3);
        assert!((interpolate_angles(-0.1, 2.3) - 1.1).abs() < 1.0e-3);
        assert!((interpolate_angles(-3.0, 3.0) - PI).abs() < 1.0e-3);
    }
}