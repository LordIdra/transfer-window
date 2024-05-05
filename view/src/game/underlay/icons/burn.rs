use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::ComponentType, storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::{burn::BurnState, Selected}, Scene};

use super::Icon;

#[derive(Debug)]
pub struct Burn {
    entity: Entity,
    time: f64,
}

impl Burn {
    pub fn generate(model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in model.entities(vec![ComponentType::VesselComponent]) {
            // Find the last burn that's not in progress
            // Previous burns will have a locked icon so we don't want to render those
            for burn in model.path_component(entity).future_burns().iter().rev() {
                let time = burn.start_point().time();
                if time > model.time() {
                    let icon = Self { entity, time };
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                    break
                }
            }
        }
        icons
    }
}

impl Icon for Burn {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        "burn".to_string()
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
        10.0
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
        let _span = tracy_client::span!("Burn position");
        let burn = model.burn_starting_at_time(self.entity, self.time);
        model.absolute_position(burn.parent()) + burn.start_point().position()
    }

    fn facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        if let Selected::Burn { entity, time, state: _ } = &view.selected {
            *entity == self.entity && *time == self.time
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &mut Scene, _model: &Model, pointer: &PointerState) {
        if !pointer.primary_clicked() {
            return;
        }
        
        if let Selected::Burn { entity, time, state } = &mut view.selected {
            if *entity == self.entity && *time == self.time {
                if state.is_selected() {
                    trace!("Burn icon clicked; switching Selected -> Adjusting");
                    *state = BurnState::Adjusting;
                } else if state.is_adjusting() {
                    trace!("Burn icon clicked; switching Adjusting -> Selected");
                    *state = BurnState::Selected;
                }
                return;
            }
        }

        trace!("Burn icon clicked; switching to Selected");
        view.selected = Selected::Burn { entity: self.entity, time: self.time, state: BurnState::Selected }
    }
}