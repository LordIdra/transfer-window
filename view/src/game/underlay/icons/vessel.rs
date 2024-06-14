use eframe::egui::PointerState;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{vessel_component::{Faction, VesselClass}, ComponentType}, storage::entity_allocator::Entity};

use crate::game::{events::ViewEvent, selected::Selected, View};

use super::Icon;


#[derive(Debug)]
pub struct Vessel {
    entity: Entity
}

impl Vessel {
    pub fn generate(view: &View) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.entities_should_render(vec![ComponentType::VesselComponent]) {
            if !view.model.vessel_component(entity).is_ghost() {
                let icon = Self { entity };
                icons.push(Box::new(icon) as Box<dyn Icon>);
            }
        }
        icons
    }
}

impl Icon for Vessel {
    fn texture(&self, view: &View) -> String {
        let mut base_name = match view.model.vessel_component(self.entity).class() {
            VesselClass::Torpedo => "vessel-icon-torpedo",
            VesselClass::Light => "vessel-icon-light",
        }.to_string();
        if let Some(target) = view.selected.target(&view.model) {
            let selected_faction = view.model.vessel_component(view.selected.entity(&view.model).unwrap()).faction();
            if target == self.entity && Faction::Player.has_intel_for(selected_faction) {
                base_name += "-target";
            }
        }
        base_name
    }

    fn alpha(&self, _view: &View, is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            return 0.4;
        }
        if is_selected {
            return 1.0;
        }
        if is_hovered {
            return 0.8
        }
        0.6
    }

    fn radius(&self, _view: &View) -> f64 {
        10.0
    }

    fn priorities(&self, view: &View) -> [u64; 4] {
        [
            u64::from(self.is_selected(view)),
            1,
            0,
            view.model.mass(self.entity) as u64
        ]
    }

    fn position(&self, view: &View) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Vessel position");
        view.model.absolute_position(self.entity)
    }

    fn facing(&self, view: &View) -> Option<DVec2> {
        Some(view.model.velocity(self.entity).normalize())
    }

    fn is_selected(&self, view: &View) -> bool {
        if let Selected::Vessel(entity) = view.selected {
            entity == self.entity
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &View, pointer: &PointerState) {
        if pointer.primary_clicked() {
            let selected = Selected::Vessel(self.entity);
            view.add_view_event(ViewEvent::SetSelected(selected));
        } else if pointer.secondary_clicked() {
            view.add_view_event(ViewEvent::ToggleRightClickMenu(self.entity));
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}