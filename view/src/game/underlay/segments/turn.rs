use eframe::egui::Rgba;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{path_component::turn::Turn, vessel_component::faction::Faction}, storage::entity_allocator::Entity};

use crate::game::View;

const INITIAL_POINT_COUNT: usize = 50;
const TESSELLATION_THRESHOLD: f64 = 1.0e-3;
const EXTRA_MIN_DISTANCE: f64 = 1.0e-3;

pub fn compute_color(view: &View, entity: Entity) -> Rgba {
    let faction = view.model.vessel_component(entity).faction();
    let rgb = match faction {
        Faction::Player => Rgba::from_rgb(0.3, 0.3, 1.0),
        Faction::Ally => Rgba::from_rgb(0.3, 1.0, 0.3),
        Faction::Enemy => Rgba::from_rgb(1.0, 0.3, 0.3),
    };

    let alpha = if view.is_selected(entity) {
        1.0
    } else {
        0.7
    };

    Rgba::from_rgba_unmultiplied(rgb.r(), rgb.g(), rgb.b(), alpha)
}

/// Uses triangle heuristic as described in <https://www.kerbalspaceprogram.com/news/dev-diaries-orbit-tessellation>
pub fn tessellate(turn: &Turn, mut points: Vec<(f64, DVec2)>, absolute_parent_position: DVec2, camera_centre: DVec2, zoom: f64) -> Vec<(f64, DVec2)> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Tessellate");

    let to_screen_space = |point: DVec2| (point - camera_centre) * zoom;

    let mut i = 0;
    while i < points.len() - 2 {
        let point1 = points[i];
        let point2 = points[i+1];
        let point3 = points[i+2];

        let point1_screen_space = to_screen_space(point1.1);
        let point2_screen_space = to_screen_space(point2.1);
        let point3_screen_space = to_screen_space(point3.1);
        
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
            let new_time_1 = (point1.0 + point2.0) / 2.0;
            let new_time_2 = (point2.0 + point3.0) / 2.0;
            let new_position_1 = turn.point_at_time(new_time_1).position();
            let new_position_2 = turn.point_at_time(new_time_2).position();

            points.insert(i + 1, (new_time_1, absolute_parent_position + new_position_1));
            points.insert(i + 3, (new_time_2, absolute_parent_position + new_position_2));
        } else {
            i += 1;
        }
    }

    points
}

fn find_initial_points(turn: &Turn, absolute_parent_position: DVec2) -> Vec<(f64, DVec2)> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Find initial turn points");
    let start_time = turn.current_point().time();
    let time_to_step_through = turn.remaining_time();

    let mut initial_points = vec![];
    for i in 0..=INITIAL_POINT_COUNT {
        let time = start_time + (i as f64 / INITIAL_POINT_COUNT as f64) * time_to_step_through;
        initial_points.push((time, absolute_parent_position + turn.point_at_time(time).position()));
    }

    initial_points
}

pub fn compute_points(turn: &Turn, absolute_parent_position: DVec2, camera_centre: DVec2, zoom: f64) -> Vec<DVec2> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Compute guidance points");
    let points = find_initial_points(turn, absolute_parent_position);
    let points = tessellate(turn, points, absolute_parent_position, camera_centre, zoom);
    let mut new_points = vec![];
    for (_, position) in points {
        new_points.push(position);
    }
    new_points
}
