use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{vessel_component::faction::Faction, ComponentType}, model::state_query::StateQuery, storage::entity_allocator::Entity};

use crate::game::{events::ViewEvent, selected::Selected, util::should_render_at_time, View};

use super::Icon;

#[derive(Debug)]
pub struct Turn {
    entity: Entity,
    time: f64,
}

impl Turn {
    pub fn generate(view: &View) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.entities_should_render(vec![ComponentType::VesselComponent, ComponentType::PathComponent]) {
            let faction = view.model.vessel_component(entity).faction();
            if !Faction::Player.has_intel_for(faction) {
                continue;
            }
            for event in view.model.vessel_component(entity).timeline().events() {
                if event.is_start_turn() && should_render_at_time(view, entity, event.time()) {
                    let icon = Self { entity, time: event.time() };
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }
        icons
    }
}

impl Icon for Turn {
    fn texture(&self, _view: &View) -> String {
        "turn".to_string()
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
        18.0
    }

    fn priorities(&self, view: &View) -> [u64; 4] {
        [
            u64::from(self.is_selected(view)),
            0,
            5,
            view.model.mass(self.entity) as u64
        ]
    }

    fn position(&self, view: &View) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Turn position");
        let turn = view.model.path_component(self.entity)
            .future_segment_starting_at_time(self.time)
            .expect("Selected turn does not exist")
            .as_turn()
            .expect("Selected turn is not turn segment");
        view.model.absolute_position(turn.parent()) + turn.start_point().position()
    }

    fn facing(&self, _view: &View) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &View) -> bool {
        if let Selected::Turn { entity, time } = &view.selected {
            *entity == self.entity && *time == self.time
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &View, pointer: &PointerState) {
        if !pointer.primary_clicked() {
            return;
        }

        trace!("Turn icon clicked; switching to Selected");
        let selected = Selected::Turn { entity: self.entity, time: self.time };
        view.add_view_event(ViewEvent::SetSelected(selected));
    }

    fn selectable(&self) -> bool {
        true
    }
}
