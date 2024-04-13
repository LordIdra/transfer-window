use eframe::egui::Rgba;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::ComponentType, storage::entity_allocator::Entity, Model};

use crate::game::Scene;

use super::Icon;

#[derive(Debug)]
pub struct Orbitable {
    entity: Entity,
}

impl Orbitable {
    pub fn generate(model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in model.get_entities(vec![ComponentType::OrbitableComponent]) {
            let icon = Self { entity };
            icons.push(Box::new(icon) as Box<dyn Icon>);
        }
        icons
    }
}

impl Icon for Orbitable {
    fn get_texture(&self) -> &str {
        "planet"
    }

    fn get_color(&self) -> eframe::egui::Rgba {
        Rgba::from_rgb(1.0, 1.0, 1.0)
    }

    fn get_radius(&self) -> f64 {
        10.0
    }

    fn get_priorities(&self, view: &Scene, model: &Model) -> [u64; 4] {
        [
            0,
            u64::from(self.is_selected(view, model)),
            2,
            (model.get_mass_component(self.entity).get_mass() / 1.0e20) as u64
        ]
    }

    fn get_position(&self, _view: &Scene, model: &Model) -> DVec2 {
        model.get_absolute_position(self.entity)
    }

    fn get_facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        if let Some(focus) = view.camera.get_focus() {
            focus == self.entity
        } else {
            false
        }
    }

    fn on_clicked(&self, view: &mut Scene, _model: &Model) {
        view.camera.reset_panning();
        view.camera.set_focus(Some(self.entity));
    }
}