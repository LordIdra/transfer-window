use eframe::egui::PointerState;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{vessel_component::timeline::TimelineEvent, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::Scene;

use super::Icon;

const RADIUS: f64 = 12.0;

#[derive(Debug)]
pub struct Intercept {
    position: DVec2
}

impl Intercept {
    fn new(view: &Scene, model: &Model, entity: Entity, time: f64) -> Self {
        let parent = model.path_component(entity).future_segment_at_time(time).parent();
        // multiply RADIUS to bring the icon down slightly
        let offset = vec2(0.0, RADIUS * 0.8 / view.camera.zoom());
        let position = model.absolute_position(parent) + model.position_at_time(entity, time) + offset;
        Self { position }
    }

    pub fn generate(view: &Scene, model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in model.entities(vec![ComponentType::VesselComponent]) {
            if let Some(TimelineEvent::Intercept(intercept)) = model.vessel_component(entity).timeline().last_event() {
                let icon = Self::new(view, model, entity, intercept.time());
                icons.push(Box::new(icon) as Box<dyn Icon>);
            }
        }

        icons
    }
}

impl Icon for Intercept {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        "intercept".to_string()
    }

    fn alpha(&self, _view: &Scene, _model: &Model, _is_selected: bool, _is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            0.2
        } else {
            1.0
        }
    }

    fn radius(&self, _view: &Scene, _model: &Model) -> f64 {
        10.0
    }

    fn priorities(&self, _view: &Scene, _model: &Model) -> [u64; 4] {
        [
            0,
            0,
            0,
            0,
        ]
    }

    fn position(&self, _view: &Scene, _model: &Model) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Intercept position");
        self.position
    }

    fn facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, _view: &Scene, _model: &Model) -> bool {
        false
    }

    fn on_mouse_over(&self, _view: &mut Scene, _model: &Model, _pointer: &PointerState) {}
}