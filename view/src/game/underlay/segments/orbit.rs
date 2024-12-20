use std::f64::consts::PI;
use eframe::{egui::Rgba, emath::normalized_angle};
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{path_component::orbit::Orbit, vessel_component::faction::Faction}, storage::entity_allocator::Entity};

use crate::game::View;

const INITIAL_POINT_COUNT: usize = 50;
const TESSELLATION_THRESHOLD: f64 = 1.0e-4;
const EXTRA_MIN_DISTANCE: f64 = 1.0e-3;

pub fn compute_color_vessel(view: &View, entity: Entity) -> Rgba {
    let faction = view.model.vessel_component(entity).faction();
    let rgb = match faction {
        Faction::Player => Rgba::from_rgb(0.0, 1.0, 1.0),
        Faction::Ally => Rgba::from_rgb(0.6, 1.0, 0.6),
        Faction::Enemy => Rgba::from_rgb(1.0, 0.5, 0.0),
    };

    let alpha = if view.is_selected(entity) {
        1.0
    } else {
        0.7
    };

    Rgba::from_rgba_unmultiplied(rgb.r(), rgb.g(), rgb.b(), alpha)
}

pub fn compute_color_orbitable(view: &View, entity: Entity) -> Rgba {
    let alpha = if view.is_selected(entity) {
        255
    } else {
        160
    };
    Rgba::from_srgba_unmultiplied(160, 160, 160, alpha)
}

/// Uses triangle heuristic as described in <https://www.kerbalspaceprogram.com/news/dev-diaries-orbit-tessellation>
pub fn tessellate(interpolate: impl Fn(DVec2, DVec2) -> DVec2, mut points: Vec<DVec2>, absolute_parent_position: DVec2, camera_centre: DVec2, zoom: f64) -> Vec<DVec2> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Tessellate");

    let to_screen_space = |point: DVec2| (point - camera_centre) * zoom;

    let mut i = 0;
    while i < points.len() - 2 {
        let point1 = points[i];
        let point2 = points[i+1];
        let point3 = points[i+2];

        let point1_screen_space = to_screen_space(point1);
        let point2_screen_space = to_screen_space(point2);
        let point3_screen_space = to_screen_space(point3);
        
        // Heron's method, see https://www.mathopenref.com/heronsformula.html
        let a = (point1_screen_space - point2_screen_space).magnitude();
        let b = (point1_screen_space - point3_screen_space).magnitude();
        let c = (point2_screen_space - point3_screen_space).magnitude();
        let p = (a + b + c) / 2.0;
        let area = f64::sqrt(p * (p - a) * (p - b) * (p - c));
        
        // If the min distance is very small, area / min_distance can get very large, causing tessellation loops
        // We add EXTRA_MIN_DISTANCE to make sure this doesn't happen 
        let min_distance = EXTRA_MIN_DISTANCE + f64::min(point1_screen_space.magnitude_squared(), f64::min(point2_screen_space.magnitude_squared(), point3_screen_space.magnitude_squared()));

        if area / min_distance > TESSELLATION_THRESHOLD {
            let new_point_1 = interpolate(point1 - absolute_parent_position, point2 - absolute_parent_position);
            let new_point_2 = interpolate(point2 - absolute_parent_position, point3 - absolute_parent_position);

            points.insert(i + 1, absolute_parent_position + new_point_1);
            points.insert(i + 3, absolute_parent_position + new_point_2);
        } else {
            i += 1;
        }
    }

    points
}

fn find_initial_points(orbit: &Orbit, absolute_parent_position: DVec2) -> Vec<DVec2> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Find initial orbit points");
    let start_angle = normalized_angle(orbit.current_point().theta() as f32);
    let angle_to_rotate_through = orbit.remaining_angle();

    // First and last points are be equal to make sure we draw lines through the whole segment
    let mut initial_points = vec![];
    for i in 0..=INITIAL_POINT_COUNT {
        let theta = start_angle as f64 + (i as f64 / INITIAL_POINT_COUNT as f64) * angle_to_rotate_through;
        initial_points.push(absolute_parent_position + orbit.position_from_theta(theta));
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
    let _span = tracy_client::span!("Compute orbit points");
    let points = find_initial_points(orbit, absolute_parent_position);
    let interpolate = |point1: DVec2, point2: DVec2| {
        let theta_1 = f64::atan2(point1.y, point1.x);
        let theta_2 = f64::atan2(point2.y, point2.x);
        orbit.position_from_theta(interpolate_angles(theta_1, theta_2))
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