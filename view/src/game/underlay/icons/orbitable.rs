use eframe::egui::PointerState;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{orbitable_component::OrbitableType, ComponentType}, storage::entity_allocator::Entity};

use crate::game::{events::ViewEvent, selected::Selected, View};

use super::Icon;

#[derive(Debug)]
pub struct Orbitable {
    entity: Entity,
}

impl Orbitable {
    pub fn generate(view: &View) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.entities_should_render(vec![ComponentType::OrbitableComponent]) {
            let icon = Self { entity };
            icons.push(Box::new(icon) as Box<dyn Icon>);
        }
        icons
    }
}

impl Icon for Orbitable {
    fn texture(&self, view: &View) -> String {
        let mut texture = match view.model.orbitable_component(self.entity).type_() {
            OrbitableType::Star => "star",
            OrbitableType::Planet => "planet",
            OrbitableType::Moon => "moon",
        }.to_string();

        if let Selected::Vessel(entity) = view.selected {
            if let Some(target) = view.model.vessel_component(entity).target() {
                if target == self.entity {
                    texture += "-target";
                }
            }
        }
        
        texture
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
        16.0
    }

    fn priorities(&self, view: &View) -> [u64; 4] {
        [
            u64::from(self.is_selected(view)),
            2,
            0,
            (view.model.mass(self.entity) / 1.0e20) as u64
        ]
    }

    fn position(&self, view: &View) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("orbitable position");
        view.model.absolute_position(self.entity)
    }

    fn facing(&self, _view: &View) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &View) -> bool {
        if let Selected::Orbitable(entity) = view.selected {
            entity == self.entity
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &View, pointer: &PointerState) {
        if pointer.primary_clicked() {
            let selected = Selected::Orbitable(self.entity);
            view.add_view_event(ViewEvent::SetSelected(selected));
        } else if pointer.secondary_clicked() {
            view.add_view_event(ViewEvent::ToggleRightClickMenu(self.entity));
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}