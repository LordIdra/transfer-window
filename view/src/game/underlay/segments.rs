use eframe::egui::Rgba;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{trajectory_component::segment::Segment, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::{util::add_triangle, Scene};

mod burn;
mod orbit;

const RADIUS: f64 = 0.8;
fn get_orbit_color(index: usize) -> Rgba {
    let colors = vec![
        Rgba::from_srgba_premultiplied(0, 150, 255, 255),
        Rgba::from_srgba_premultiplied(0, 200, 255, 255),
        Rgba::from_srgba_premultiplied(150, 205, 220, 255),
        Rgba::from_srgba_premultiplied(75, 170, 200, 255),
        Rgba::from_srgba_premultiplied(30, 130, 180, 255),
    ];
    colors[index % (colors.len() - 1)]
}

fn get_burn_color(index: usize) -> Rgba {
    let colors = vec![
        Rgba::from_srgba_premultiplied(255, 0, 0, 255),
        Rgba::from_srgba_premultiplied(220, 110, 100, 255),
        Rgba::from_srgba_premultiplied(200, 55, 35, 255),
        Rgba::from_srgba_premultiplied(160, 50, 40, 255),
        Rgba::from_srgba_premultiplied(235, 65, 65, 255),
    ];
    colors[index % (colors.len() - 1)]
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
    let zoom = view.camera.get_zoom();
    let trajectory_component = model.get_trajectory_component(entity);
    let mut orbit_index = trajectory_component.get_previous_orbits();
    let mut burn_index = trajectory_component.get_previous_burns();
    // Reverse to make sure that the segments are rendered in order
    // of how soon they are, so that closer segments take priority
    // over further ones
    for segment in trajectory_component.get_segments().iter().rev().flatten() {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Draw segment");
        let absolute_parent_position = model.get_absolute_position(segment.get_parent());
        match segment {
            Segment::Orbit(orbit) => {
                let color = get_orbit_color(orbit_index);
                draw_from_points(view, &orbit::compute_points(orbit, absolute_parent_position, camera_centre, zoom), zoom, color);
                orbit_index += 1;
            },
            Segment::Burn(burn) => {
                let color = get_burn_color(burn_index);
                draw_from_points(view, &burn::compute_points(burn, absolute_parent_position, camera_centre, zoom), zoom, color);
                burn_index += 1;
            }
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