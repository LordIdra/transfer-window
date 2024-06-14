use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{vessel_component::{timeline::TimelineEvent, Faction}, ComponentType}, storage::entity_allocator::Entity};

use crate::game::{events::ViewEvent, selected::Selected, util::should_render_at_time, View};

use super::Icon;

#[derive(Debug)]
pub struct Intercept {
    entity: Entity,
    time: f64,
}

impl Intercept {
    pub fn generate(view: &View) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.entities_should_render(vec![ComponentType::VesselComponent]) {
            let faction = view.model.vessel_component(entity).faction();
            if !Faction::Player.has_intel_for(faction) {
                continue;
            }
            if let Some(TimelineEvent::Intercept(intercept)) = view.model.vessel_component(entity).timeline().last_event() {
                if should_render_at_time(view, entity, intercept.time()) {
                    let icon = Self { entity, time: intercept.time() };
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }

        icons
    }
}

impl Icon for Intercept {
    fn texture(&self, _view: &View) -> String {
        "intercept".to_string()
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
        8.0
    }

    fn priorities(&self, view: &View) -> [u64; 4] {
        [
            u64::from(self.is_selected(view)),
            0,
            4,
            0,
        ]
    }

    fn position(&self, view: &View) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Intercept position");
        let parent = view.model.parent_at_time(self.entity, self.time, Some(Faction::Player)).unwrap();
        view.model.absolute_position(parent) + view.model.position_at_time(self.entity, self.time, Some(Faction::Player))
    }

    fn facing(&self, _view: &View) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &View) -> bool {
        if let Selected::Intercept { entity, time } = &view.selected {
            *entity == self.entity && (*time - self.time).abs() < 1.0
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &View, pointer: &PointerState) {
        if pointer.primary_clicked() {
            trace!("Intercept icon clicked; switching to Selected");
            let selected = Selected::Intercept { entity: self.entity, time: self.time };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}