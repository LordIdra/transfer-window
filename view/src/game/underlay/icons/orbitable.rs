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

    fn get_group_priority(&self) -> usize {
        2
    }

    fn get_priority(&self, model: &Model) -> usize {
        // Divide to prevent overflow
        (model.get_mass_component(self.entity).get_mass() / 1.0e22) as usize
    }

    fn get_position(&self, model: &Model) -> DVec2 {
        model.get_absolute_position(self.entity)
    }

    fn is_selected(&self, view: &Scene) -> bool {
        if let Some(focus) = view.camera.get_focus() {
            focus == self.entity
        } else {
            false
        }
    }

    fn on_clicked(&self, view: &mut Scene) {
        view.camera.reset_panning();
        view.camera.set_focus(Some(self.entity));
    }
}