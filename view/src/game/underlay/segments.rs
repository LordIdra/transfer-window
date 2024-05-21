use eframe::egui::Rgba;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{orbitable_component::OrbitableComponentPhysics, path_component::segment::Segment, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::{util::add_triangle, Scene};

mod burn;
mod guidance;
mod orbit;

const RADIUS: f64 = 0.8;

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

fn draw_path_segments(view: &mut Scene, model: &Model, entity: Entity, camera_centre: DVec2) {
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
                let color = orbit::compute_color_vessel(view, model, entity, orbit_index);
                segment_points_data.push((points, color));
                orbit_index += 1;
            },

            Segment::Burn(burn) => {
                let points = burn::compute_points(burn, absolute_parent_position, camera_centre, zoom);
                let color = burn::compute_color();
                segment_points_data.push((points, color));
                orbit_index = 0;
            }

            Segment::Guidance(guidance) => {
                let points = guidance::compute_points(guidance, absolute_parent_position, camera_centre, zoom);
                let color = guidance::compute_color();
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

fn draw_orbitable_segment(view: &mut Scene, model: &Model, entity: Entity, camera_centre: DVec2) {
    let orbitable_component = model.orbitable_component(entity);
    if let OrbitableComponentPhysics::Orbit(orbit) = orbitable_component.physics() {
        let absolute_parent_position = model.absolute_position(orbit.parent());
        let zoom = view.camera.zoom();
        let points = orbit::compute_points(orbit, absolute_parent_position, camera_centre, zoom);
        let color = orbit::compute_color_orbitable(view, model, entity);
        draw_from_points(view, &points, zoom, color);
    }
}

pub fn draw(view: &mut Scene, model: &Model) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw segments");
    let camera_centre = view.camera.translation(model);
    for entity in model.entities(vec![ComponentType::PathComponent]) {
        draw_path_segments(view, model, entity, camera_centre);
    }
    for entity in model.entities(vec![ComponentType::OrbitableComponent]) {
        draw_orbitable_segment(view, model, entity, camera_centre);
    }
}