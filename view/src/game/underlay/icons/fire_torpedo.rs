use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::ComponentType, storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::{util::BurnState, Selected}, Scene};

use super::Icon;

#[derive(Debug)]
pub struct FireTorpedo {
    entity: Entity,
    time: f64,
}

impl FireTorpedo {
    pub fn generate(view: &Scene, model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.entities_should_render(model, vec![ComponentType::VesselComponent]) {
            for event in model.vessel_component(entity).timeline().events() {
                if event.is_fire_torpedo() {
                    let icon = Self { entity, time: event.time() };
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }
        icons
    }
}

impl Icon for FireTorpedo {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        "torpedo".to_string()
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
        18.0
    }

    fn priorities(&self, view: &Scene, model: &Model) -> [u64; 4] {
        [
            u64::from(self.is_selected(view, model)),
            0,
            0,
            model.mass(self.entity) as u64
        ]
    }

    fn position(&self, _view: &Scene, model: &Model) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Fire torpedo position");
        let orbit = model.orbit_at_time(self.entity, self.time);
        model.absolute_position(orbit.parent()) + orbit.point_at_time(self.time).position()
    }

    fn facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        if let Selected::FireTorpedo { entity, time, state: _ } = &view.selected {
            *entity == self.entity && *time == self.time
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &mut Scene, model: &Model, pointer: &PointerState) {
        if !pointer.primary_clicked() {
            return;
        }
        
        if let Selected::FireTorpedo { entity, time, state } = &mut view.selected {
            if *entity == self.entity 
                    && *time == self.time 
                    && model.timeline_event_at_time(self.entity, self.time).can_adjust(model)
                    && model.timeline_event_at_time(*entity, *time).as_fire_torpedo().unwrap().can_adjust(model) {
                if state.is_selected() {
                    trace!("Fire torpedo icon clicked; switching Selected -> Adjusting");
                    *state = BurnState::Adjusting;
                } else if state.is_adjusting() {
                    trace!("Fire torpedo icon clicked; switching Adjusting -> Selected");
                    *state = BurnState::Selected;
                }
                return;
            }
        }

        trace!("Fire torpedo icon clicked; switching to Selected");
        view.selected = Selected::FireTorpedo { entity: self.entity, time: self.time, state: BurnState::Selected }
    }

    fn selectable(&self) -> bool {
        true
    }
}