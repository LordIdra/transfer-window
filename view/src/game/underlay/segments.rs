use eframe::egui::Rgba;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{path_component::segment::Segment, ComponentType}, storage::entity_allocator::Entity, Model, SEGMENTS_TO_PREDICT};

use crate::game::{util::add_triangle, Scene};

mod burn;
mod orbit;

const RADIUS: f64 = 0.8;

fn compute_orbit_color(view: &Scene, model: &Model, entity: Entity, index: usize) -> Rgba {
    let is_vessel = model.try_vessel_component(entity).is_some();
    let is_selected = if let Some(selected) = view.selected.selected_entity() {
        selected == entity
    } else {
        false
    };

    let colors: [Rgba; SEGMENTS_TO_PREDICT] = if is_vessel {
        if is_selected {
            [
                Rgba::from_srgba_unmultiplied(0, 255, 255, 255),
                Rgba::from_srgba_unmultiplied(0, 255, 255, 170),
                Rgba::from_srgba_unmultiplied(0, 255, 255, 130),
                Rgba::from_srgba_unmultiplied(0, 255, 255, 100),
            ]
        } else {
            [Rgba::from_srgba_unmultiplied(0, 255, 255, 60); SEGMENTS_TO_PREDICT]
        }
    } else if is_selected {
        [Rgba::from_srgba_unmultiplied(255, 255, 255, 160); SEGMENTS_TO_PREDICT]
    } else {
        [Rgba::from_srgba_unmultiplied(255, 255, 255, 50); SEGMENTS_TO_PREDICT]
    };

    colors[index]
}

fn compute_burn_color() -> Rgba {
    Rgba::from_srgba_premultiplied(255, 255, 255, 255)
}

/// Draws a line between two points so that all the lines on a segment are connected together
/// This should be called multiple times with different i's to create a blur effect, where i represents how far away from the 'real' line this line is
fn add_orbit_line(vertices: &mut Vec<f32>, previous_point: &DVec2, new_point: &DVec2, zoom: f64, color: Rgba) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Add orbit line");
    let radius = RADIUS;

    let direction_unit = (new_point - previous_point).normalize();
    let perpendicular_unit = vec2(-direction_unit.y, direction_unit.x);

    let v1 = previous_point + (perpendicular_unit * radius / zoom);
    let v2 = previous_point - (perpendicular_unit * radius / zoom);
    let v3 = new_point + (perpendicular_unit * radius / zoom);
    let v4 = new_point - (perpendicular_unit * radius / zoom);

    add_triangle(vertices, v1, v2, v3, color);
    add_triangle(vertices, v2, v3, v4, color);
}

fn draw_from_points(view: &mut Scene, points: &[DVec2], zoom: f64, color: Rgba) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw from points");
    let mut vertices = vec![];
    let mut previous_point = None;
    for new_point in points {
        if let Some(previous_point) = previous_point {
            add_orbit_line(&mut vertices, previous_point, new_point, zoom, color);
        }
        previous_point = Some(new_point);
    }
    view.segment_renderer.lock().unwrap().add_vertices(&mut vertices);
}

fn draw_entity_segments(view: &mut Scene, model: &Model, entity: Entity, camera_centre: DVec2) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw segments for one entity");
    let zoom = view.camera.zoom();
    let path_component = model.path_component(entity);

    let mut segment_points_data = vec![];
    let mut orbit_index = 0;
    for segment in path_component.future_segments() {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Draw segment");
        let absolute_parent_position = model.absolute_position(segment.parent());
        match segment {
            Segment::Orbit(orbit) => {
                // When predicting trajectories, the last orbit will have duration zero, so skip it
                if orbit.duration().abs() == 0.0 {
                    continue;
                }
                let points = orbit::compute_points(orbit, absolute_parent_position, camera_centre, zoom);
                let color = compute_orbit_color(view, model, entity, orbit_index);
                segment_points_data.push((points, color));
                orbit_index += 1;
            },
            Segment::Burn(burn) => {
                let points = burn::compute_points(burn, absolute_parent_position, camera_centre, zoom);
                let color = compute_burn_color();
                segment_points_data.push((points, color));
                orbit_index = 0;
            }
        };
    }

    // Reverse to make sure that the segments are rendered in order
    // of how soon they are, so that closer segments take priority
    // over further ones
    for (segment_points, color) in segment_points_data.iter().rev() {
        draw_from_points(view, segment_points, zoom, *color);
    }
}

pub fn draw(view: &mut Scene, model: &Model) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw segments");
    let camera_centre = view.camera.translation(model);
    for entity in model.entities(vec![ComponentType::PathComponent]) {
        draw_entity_segments(view, model, entity, camera_centre);
    }
}