use std::f64::consts::TAU;

use nalgebra_glm::{vec2, DVec2, Vec2, convert};
use transfer_window_model::components::ComponentType;

use crate::game::{util::add_textured_triangle, View};

fn compute_celestial_object_vertices(absolute_position: DVec2, radius: f64) -> Vec<f32> {
    let scaled_radius = radius;
    let mut vertices = vec![];
    let sides = 100;
    let mut previous_location = absolute_position + vec2(scaled_radius, 0.0);
    for i in 1..=sides { // 1..=sides to make sure we fill in the gap between the last location and first location, wrapping back round
        let angle = (i as f64 / sides as f64) * TAU; // both i and sides must be cast to prevent integer division problems
        let new_location = absolute_position + vec2(f64::cos(angle), f64::sin(angle)) * scaled_radius;
        add_textured_triangle(
            &mut vertices,
            absolute_position,
            previous_location,
            new_location,
            1.0,
            vec2(0.5, 0.5),
            vert_to_uv(previous_location, absolute_position, scaled_radius),
            vert_to_uv(new_location, absolute_position, scaled_radius),
        );
        previous_location = new_location;
    }
    vertices
}

fn vert_to_uv(vert: DVec2, center: DVec2, radius: f64) -> Vec2 {
    let diff = vert - center;
    convert(diff / (2.0 * radius) + vec2(0.5, 0.5))
}

pub fn draw(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw celestial objects");
    for entity in view.entities_should_render(vec![ComponentType::OrbitableComponent]) {
        let position = view.model.absolute_position(entity);
        let radius = view.model.orbitable_component(entity).radius();
        let name = view.model.name_component(entity).name().to_lowercase();
        let mut vertices = compute_celestial_object_vertices(position, radius);
        view.renderers.add_object_vertices(&name, &mut vertices);
    }
}