use eframe::egui::Rgba;
use nalgebra_glm::DVec2;
use transfer_window_model::components::path_component::guidance::Guidance;

const INITIAL_POINT_COUNT: usize = 50;
const TESSELLATION_THRESHOLD: f64 = 1.0e-3;
const EXTRA_MIN_DISTANCE: f64 = 1.0e-3;

pub fn compute_color() -> Rgba {
    Rgba::from_srgba_premultiplied(0, 255, 200, 255)
}

/// Uses triangle heuristic as described in <https://www.kerbalspaceprogram.com/news/dev-diaries-orbit-tessellation>
pub fn tessellate(guidance: &Guidance, mut points: Vec<(f64, DVec2)>, absolute_parent_position: DVec2, camera_centre: DVec2, zoom: f64) -> Vec<(f64, DVec2)> {
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
            let new_position_1 = guidance.point_at_time(new_time_1).position();
            let new_position_2 = guidance.point_at_time(new_time_2).position();

            points.insert(i + 1, (new_time_1, absolute_parent_position + new_position_1));
            points.insert(i + 3, (new_time_2, absolute_parent_position + new_position_2));
        } else {
            i += 1;
        }
    }

    points
}

fn find_initial_points(guidance: &Guidance, absolute_parent_position: DVec2) -> Vec<(f64, DVec2)> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Find initial burn points");
    let start_time = guidance.current_point().time();
    let time_to_step_through = guidance.remaining_time();

    let mut initial_points = vec![];
    for i in 0..=INITIAL_POINT_COUNT {
        let time = start_time + (i as f64 / INITIAL_POINT_COUNT as f64) * time_to_step_through;
        initial_points.push((time, absolute_parent_position + guidance.point_at_time(time).position()));
    }

    initial_points
}

pub fn compute_points(guidance: &Guidance, absolute_parent_position: DVec2, camera_centre: DVec2, zoom: f64) -> Vec<DVec2> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Compute guidance points");
    let points = find_initial_points(guidance, absolute_parent_position);
    let points = tessellate(guidance, points, absolute_parent_position, camera_centre, zoom);
    let mut new_points = vec![];
    for (_, position) in points {
        new_points.push(position);
    }
    new_points
}