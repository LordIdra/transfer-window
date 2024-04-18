use eframe::egui::PointerState;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::ComponentType, storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::Selected, Scene};

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
    fn get_texture(&self, view: &Scene, model: &Model) -> String {
        if let Selected::Vessel(entity) = view.selected {
            if let Some(target) = model.get_vessel_component(entity).get_target() {
                if target == self.entity {
                    return "planet-target".to_string()
                }
            }
        }
        "planet".to_string()
    }

    fn get_alpha(&self, _view: &Scene, _model: &Model, is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            return 0.2;
        }
        if is_selected {
            return 1.0;
        }
        if is_hovered {
            return 0.7
        }
        0.4
    }

    fn get_radius(&self, _view: &Scene, _model: &Model) -> f64 {
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
        if let Selected::Orbitable(entity) = view.selected {
            entity == self.entity
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &mut Scene, _model: &Model, pointer: &PointerState) {
        if pointer.primary_clicked() {
            view.selected = Selected::Orbitable(self.entity);
        } else if pointer.secondary_clicked() {
            view.right_click_menu = Some(self.entity);
        }
    }
}