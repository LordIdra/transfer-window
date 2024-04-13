use eframe::egui::Rgba;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{trajectory_component::segment::Segment, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::{util::add_triangle, Scene};

mod orbit;
mod util;

const RADIUS: f64 = 0.8;

/// Draws a line between two points so that all the lines on a segment are connected together
/// This should be called multiple times with different i's to create a blur effect, where i represents how far away from the 'real' line this line is
fn add_orbit_line(vertices: &mut Vec<f32>, previous_point: &DVec2, new_point: &DVec2, zoom: f64) {
    let radius = RADIUS;
    let rgba = Rgba::from_rgba_unmultiplied(0.0, 0.6, 1.0, 1.0);

    let direction_unit = (new_point - previous_point).normalize();
    let perpendicular_unit = vec2(-direction_unit.y, direction_unit.x);

    let v1 = previous_point + (perpendicular_unit * radius / zoom);
    let v2 = previous_point - (perpendicular_unit * radius / zoom);
    let v3 = new_point + (perpendicular_unit * radius / zoom);
    let v4 = new_point - (perpendicular_unit * radius / zoom);

    add_triangle(vertices, v1, v2, v3, rgba);
    add_triangle(vertices, v2, v3, v4, rgba);
}

fn draw_from_points(view: &mut Scene, points: &[DVec2], zoom: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw from points");
    let mut vertices = vec![];
    let mut previous_point = None;
    for new_point in points {
        if let Some(previous_point) = previous_point {
            add_orbit_line(&mut vertices, previous_point, new_point, zoom);
        }
        previous_point = Some(new_point);
    }
    view.segment_renderer.lock().unwrap().add_vertices(&mut vertices);
}

fn draw_entity_segments(view: &mut Scene, model: &Model, entity: Entity, camera_centre: DVec2) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw segments for one entity");
    let zoom = view.camera.get_zoom();
    let trajectory_component = model.get_trajectory_component(entity);
    for segment in trajectory_component.get_segments().iter().flatten() {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Draw segment");
        let absolute_parent_position = model.get_absolute_position(segment.get_parent());
        match segment {
            Segment::Orbit(orbit) => draw_from_points(view, &orbit::compute_points(orbit, absolute_parent_position, camera_centre, zoom), zoom),
            Segment::Burn(_) => (), //TODO
        };
    }
}

pub fn draw(view: &mut Scene, model: &Model) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw segments");
    let camera_centre = view.camera.get_translation(model);
    for entity in model.get_entities(vec![ComponentType::TrajectoryComponent]) {
        draw_entity_segments(view, model, entity, camera_centre);
    }
}