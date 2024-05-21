use eframe::egui::PointerState;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{vessel_component::VesselClass, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::Selected, Scene};

use super::Icon;


#[derive(Debug)]
pub struct Vessel {
    entity: Entity
}

impl Vessel {
    pub fn generate(model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in model.entities(vec![ComponentType::VesselComponent]) {
            if !model.vessel_component(entity).is_ghost() {
                let icon = Self { entity };
                icons.push(Box::new(icon) as Box<dyn Icon>);
            }
        }
        icons
    }
}

impl Icon for Vessel {
    fn texture(&self, view: &Scene, model: &Model) -> String {
        let mut base_name = match model.vessel_component(self.entity).class() {
            VesselClass::Torpedo => "vessel-icon-torpedo",
            VesselClass::Light => "vessel-icon-light",
        }.to_string();
        if let Some(target) = view.selected.target(model) {
            if target == self.entity {
                base_name += "-target";
            }
        }
        base_name
    }

    fn alpha(&self, _view: &Scene, _model: &Model, is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
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

    fn radius(&self, _view: &Scene, _model: &Model) -> f64 {
        12.0
    }

    fn priorities(&self, view: &Scene, model: &Model) -> [u64; 4] {
        [
            0,
            u64::from(self.is_selected(view, model)),
            1,
            model.mass(self.entity) as u64
        ]
    }

    fn position(&self, _view: &Scene, model: &Model) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Vessel position");
        model.absolute_position(self.entity)
    }

    fn facing(&self, _view: &Scene, model: &Model) -> Option<DVec2> {
        let direction = model.velocity(self.entity).normalize();
        Some(vec2(-direction.y, direction.x))
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        if let Selected::Vessel(entity) = view.selected {
            entity == self.entity
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &mut Scene, _model: &Model, pointer: &PointerState) {
        if pointer.primary_clicked() {
            view.selected = Selected::Vessel(self.entity);
        } else if pointer.secondary_clicked() {
            view.toggle_right_click_menu(self.entity);
        }
    }
}