use eframe::egui::{Context, Rgba};
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{trajectory_component::segment::Segment, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::{util::add_triangle, Scene};

mod orbit;

const MAX_ALPHA: f32 = 0.6;
const RADIUS_DELTA: f64 = 0.6;

/// Draws a line between two points so that all the lines on a segment are connected together
/// This should be called multiple times with different i's to create a blur effect, where i represents how far away from the 'real' line this line is
fn add_orbit_line(vertices: &mut Vec<f32>, previous_point: &DVec2, new_point: &DVec2, max_alpha: f32, zoom: f64, i: i32) {
    let radius = RADIUS_DELTA + (i as f64 * RADIUS_DELTA); // Start off with non-zero radius
    let mut alpha = max_alpha;
    if i != 0 {
        // Scale the alpha non-linearly so we have lots of values close to zero
        alpha /= 7.0 * i as f32;
    }

    let rgba = Rgba::from_rgba_unmultiplied(0.0, 0.6, 1.0, alpha);

    let direction_unit = (new_point - previous_point).normalize();
    let perpendicular_unit = vec2(-direction_unit.y, direction_unit.x);

    let v1 = previous_point + (perpendicular_unit * radius / zoom);
    let v2 = previous_point - (perpendicular_unit * radius / zoom);
    let v3 = new_point + (perpendicular_unit * radius / zoom);
    let v4 = new_point - (perpendicular_unit * radius / zoom);

    add_triangle(vertices, v1, v2, v3, rgba);
    add_triangle(vertices, v2, v3, v4, rgba);
}

fn draw_section(view: &mut Scene, section: Vec<DVec2>, zoom: f64) {
    let mut vertices = vec![];
    let mut previous_point = None;
    for new_point in &section {
        if let Some(previous_point) = previous_point {
            // Loop to create glow effect
            for i in 0..10 {
                add_orbit_line(&mut vertices, previous_point, new_point, MAX_ALPHA, zoom, i);
            }
        }
        previous_point = Some(new_point);
    }
    view.segment_renderer.lock().unwrap().add_vertices(&mut vertices);
}

fn draw_entity_orbits(view: &mut Scene, model: &Model, entity: Entity, camera_centre: DVec2, radius: f64) {
    let zoom = view.camera.get_zoom();
    let trajectory_component = model.get_trajectory_component(entity);
    for segment in trajectory_component.get_segments().iter().flatten() {
        let absolute_parent_position = model.get_absolute_position(segment.get_parent());

        match segment {
            Segment::Orbit(orbit) =>{
                for section in orbit::compute_sections(orbit, absolute_parent_position, camera_centre, zoom, radius) {
                    draw_section(view, section, zoom);
                }
            }

            Segment::Burn(_) => todo!(),
        };

        
    }
}

pub fn draw(view: &mut Scene, model: &Model, context: &Context) {
    let world_width = 0.5 * context.screen_rect().width() as f64 / view.camera.get_zoom();
    let world_height = 0.5 * context.screen_rect().height() as f64 / view.camera.get_zoom();
    let camera_centre = view.camera.get_translation(model);
    let radius = f64::sqrt(world_width.powi(2) + world_height.powi(2));
    for entity in model.get_entities(vec![ComponentType::TrajectoryComponent]) {
        draw_entity_orbits(view, model, entity, camera_centre, radius);
    }
}