use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{vessel_component::timeline::TimelineEvent, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::{selected::Selected, util::should_render_at_time, Scene};

use super::Icon;

#[derive(Debug)]
pub struct Intercept {
    entity: Entity,
    time: f64,
}

impl Intercept {
    pub fn generate(view: &Scene, model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.entities_should_render(model, vec![ComponentType::VesselComponent]) {
            if let Some(TimelineEvent::Intercept(intercept)) = model.vessel_component(entity).timeline().last_event() {
                if should_render_at_time(view, model, entity, intercept.time()) {
                    let icon = Self { entity, time: intercept.time() };
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }

        icons
    }
}

impl Icon for Intercept {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        "intercept".to_string()
    }

    fn alpha(&self, _view: &Scene, _model: &Model, is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
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

    fn radius(&self, _view: &Scene, _model: &Model) -> f64 {
        8.0
    }

    fn priorities(&self, view: &Scene, model: &Model) -> [u64; 4] {
        [
            u64::from(self.is_selected(view, model)),
            0,
            4,
            0,
        ]
    }

    fn position(&self, _view: &Scene, model: &Model) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Intercept position");
        let parent = model.parent_at_time(self.entity, self.time).unwrap();
        model.absolute_position(parent) + model.position_at_time(self.entity, self.time)
    }

    fn facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        if let Selected::Intercept { entity, time } = &view.selected {
            *entity == self.entity && *time == self.time
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &mut Scene, _model: &Model, pointer: &PointerState) {
        if pointer.primary_clicked() {
            trace!("Intercept icon clicked; switching to Selected");
            view.selected = Selected::Intercept { entity: self.entity, time: self.time };
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}