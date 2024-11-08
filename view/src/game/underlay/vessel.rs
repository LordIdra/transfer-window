use nalgebra_glm::vec2;
use transfer_window_model::{components::ComponentType, model::state_query::StateQuery};

use crate::game::{util::add_textured_rectangle_facing, View};

pub fn draw(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw vessels");

    for entity in view.entities_should_render(vec![ComponentType::VesselComponent]) {
        let vessel = view.model.vessel_component(entity);
        let position = view.model.absolute_position(entity);
        let rotation = view.model.rotation(entity);
        let dimensions = vessel.dimensions();
        let facing = vec2(f64::cos(rotation), f64::sin(rotation));

        let mut vertices = vec![];
        add_textured_rectangle_facing(&mut vertices, position, dimensions, 1.0, facing);
        view.renderers.add_texture_vertices(vessel.class().name(), &mut vertices);
    }
}
