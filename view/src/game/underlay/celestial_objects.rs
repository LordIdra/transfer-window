use std::f64::consts::PI;

use eframe::egui::Rgba;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::ComponentType, Model};

use crate::game::{util::add_triangle, Scene};

fn compute_celestial_object_vertices(absolute_position: DVec2, radius: f64) -> Vec<f32> {
    let scaled_radius = radius;
    let mut vertices = vec![];
    let sides = 100;
    let mut previous_location = absolute_position + vec2(scaled_radius, 0.0);
    for i in 1..=sides { // 1..=sides to make sure we fill in the gap between the last location and first location, wrapping back round
        let angle = (i as f64 / sides as f64) * 2.0 * PI; // both i and sides must be cast to prevent integer division problems
        let new_location = absolute_position + vec2(scaled_radius * f64::cos(angle), scaled_radius * f64::sin(angle));
        add_triangle(&mut vertices, absolute_position, previous_location, new_location, Rgba::RED);
        previous_location = new_location;
    }
    vertices
}

pub fn draw(view: &mut Scene, model: &Model) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw celestial objects");
    for entity in model.entities(vec![ComponentType::OrbitableComponent]) {
        let position = model.absolute_position(entity);
        let radius = model.orbitable_component(entity).radius();
        let mut vertices = compute_celestial_object_vertices(position, radius);
        view.renderers.add_object_vertices(&mut vertices);
    }
}