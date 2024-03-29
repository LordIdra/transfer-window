use std::f64::consts::PI;

use eframe::egui::Rgba;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::ComponentType, Model};

use crate::game::{util::add_triangle, Scene};

fn get_celestial_object_vertices(position: DVec2, radius: f64) -> Vec<f32> {
    let scaled_radius = radius;
    let absolute_scaled_position = position;
    let mut vertices = vec![];
    let sides = 100;
    let mut previous_location = absolute_scaled_position + vec2(scaled_radius, 0.0);
    for i in 1..=sides { // 1..=sides to make sure we fill in the gap between the last location and first location, wrapping back round
        let angle = (i as f64 / sides as f64) * 2.0 * PI; // both i and sides must be cast to prevent integer division problems
        let new_location = absolute_scaled_position + vec2(scaled_radius * f64::cos(angle), scaled_radius * f64::sin(angle));
        add_triangle(&mut vertices, absolute_scaled_position, previous_location, new_location, Rgba::RED);
        previous_location = new_location;
    }
    vertices
}

pub fn draw(view: &Scene, model: &Model) {
    // Stationary entities
    for entity in model.get_entities(vec![ComponentType::OrbitableComponent, ComponentType::StationaryComponent]) {
        let position = model.get_stationary_component(entity).get_position();
        let radius = model.get_orbitable_component(entity).get_radius();
        let mut vertices = get_celestial_object_vertices(position, radius);
        view.object_renderer.lock().unwrap().add_vertices(&mut vertices);
    }

    // Trajectory entities
    for entity in model.get_entities(vec![ComponentType::OrbitableComponent, ComponentType::TrajectoryComponent]) {
        let position = model.get_trajectory_component(entity).get_current_segment().get_current_position();
        let radius = model.get_orbitable_component(entity).get_radius();
        let mut vertices = get_celestial_object_vertices(position, radius);
        view.object_renderer.lock().unwrap().add_vertices(&mut vertices);
    }
}