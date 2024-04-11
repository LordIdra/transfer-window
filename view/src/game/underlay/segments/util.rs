use nalgebra_glm::DVec2;

const TESSELLATION_THRESHOLD: f64 = 1.0e-4;

/// Uses triangle heuristic as described in https://www.kerbalspaceprogram.com/news/dev-diaries-orbit-tessellation
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

        let min_distance = f64::min(point1_screen_space.magnitude_squared(), f64::min(point2_screen_space.magnitude_squared(), point3_screen_space.magnitude_squared()));

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