use eframe::egui::PointerState;
use nalgebra_glm::DVec2;
use transfer_window_model::{api::encounters::EncounterType, components::ComponentType, storage::entity_allocator::Entity, Model};

use crate::game::{util::should_render_at_time, Scene};

use super::Icon;

#[derive(Debug)]
pub struct Encounter {
    encounter_type: EncounterType,
    position: DVec2,
}

impl Encounter {
    fn new(model: &Model, entity: Entity, time: f64, encounter_type: EncounterType) -> Self {
        let orbit = model.orbit_at_time(entity, time);
        let position = model.absolute_position(orbit.parent()) + orbit.point_at_time(time).position();
        Self { encounter_type, position }
    }

    pub fn generate(view: &Scene, model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in model.entities(vec![ComponentType::PathComponent]) {
            for encounter in model.future_encounters(entity) {
                if should_render_at_time(view, model, entity, encounter.time()) {
                    let icon = Self::new(model, entity, encounter.time(), encounter.encounter_type());
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }

        icons
    }
}

impl Icon for Encounter {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        if matches!(self.encounter_type, EncounterType::Entrance) {
            "encounter-entrance".to_string()
        } else {
            "encounter-exit".to_string()
        }
    }

    fn alpha(&self, _view: &Scene, _model: &Model, _is_selected: bool, _is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            0.4
        } else {
            1.0
        }
    }

    fn radius(&self, _view: &Scene, _model: &Model) -> f64 {
        8.0
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
        let _span = tracy_client::span!("Periapsis position");
        self.position
    }

    fn facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, _view: &Scene, _model: &Model) -> bool {
        false
    }

    fn on_mouse_over(&self, _view: &mut Scene, _model: &Model, _pointer: &PointerState) {}

    fn selectable(&self) -> bool {
        false
    }
}