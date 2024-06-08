use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{api::encounters::EncounterType, components::ComponentType, storage::entity_allocator::Entity};

use crate::game::{selected::Selected, util::should_render_at_time, View};

use super::Icon;

#[derive(Debug)]
pub struct Encounter {
    type_: EncounterType,
    entity: Entity,
    time: f64,
    from: Entity,
    to: Entity,
}

impl Encounter {
    pub fn generate(view: &View) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.model.entities(vec![ComponentType::PathComponent]) {
            for encounter in view.model.future_encounters(entity) {
                if should_render_at_time(view, entity, encounter.time()) {
                    let icon = Self { type_: encounter.encounter_type(), entity, time: encounter.time(), from: encounter.from(), to: encounter.to() };
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }

        icons
    }
}

impl Icon for Encounter {
    fn texture(&self, _view: &View) -> String {
        match self.type_ {
            EncounterType::Entrance => "encounter-entrance",
            EncounterType::Exit => "encounter-exit",
        }.to_string()
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
            3,
            0,
        ]
    }

    fn position(&self, view: &View) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Encounter position");
        let orbit = view.model.orbit_at_time(self.entity, self.time);
        view.model.absolute_position(orbit.parent()) + orbit.point_at_time(self.time).position()
    }

    fn facing(&self, _view: &View) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &View) -> bool {
        if let Selected::Encounter { type_, entity, time, from: _, to: _ } = &view.selected {
            *type_ == self.type_ && *entity == self.entity && *time == self.time
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &mut View, pointer: &PointerState) {
        if pointer.primary_clicked() {
            trace!("Encounter icon clicked; switching to Selected");
            view.selected = Selected::Encounter { type_: self.type_, entity: self.entity, time: self.time, from: self.from, to: self.to };
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}