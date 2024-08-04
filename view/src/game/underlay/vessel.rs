use transfer_window_model::components::{vessel_component::class::VesselClass, ComponentType};

use crate::game::{util::add_textured_square, View};

fn get_ship_size(class: VesselClass) -> f64 {
    match class {
        VesselClass::Scout1 => 100.0,
        VesselClass::Torpedo => todo!(),
        VesselClass::Hub => todo!(),
        VesselClass::Scout2 => todo!(),
        VesselClass::Frigate1 => todo!(),
        VesselClass::Frigate2 => todo!(),
    }
}

pub fn draw(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw vessels");

    for entity in view.entities_should_render(vec![ComponentType::VesselComponent]) {
        let vessel = view.model.vessel_component(entity);
        let position = view.model.absolute_position(entity);
        let radius = get_ship_size(vessel.class());

        let mut vertices = vec![];
        add_textured_square(&mut vertices, position, radius, 1.0);
        view.renderers.add_texture_vertices(vessel.class().name(), &mut vertices);
    }
}
