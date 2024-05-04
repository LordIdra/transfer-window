use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{path_component::segment::Segment, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::{burn::BurnState, Selected}, Scene};

use super::Icon;

#[derive(Debug)]
pub struct BurnLocked {
    entity: Entity,
    time: f64,
}

impl BurnLocked {
    pub fn generate(model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in model.entities(vec![ComponentType::VesselComponent]) {
            let Some(last_burn) = model.path_component(entity).final_burn() else {
                continue;
            };
            let last_burn_time = last_burn.start_point().time();
            for segment in model.path_component(entity).segments().iter().flatten().rev() {
                if let Segment::Burn(burn) = segment {
                    let time = burn.start_point().time();
                    if time > model.time() && time != last_burn_time {
                        let icon = Self { entity, time };
                        icons.push(Box::new(icon) as Box<dyn Icon>);
                    }
                }
            }
        }
        icons
    }
}

impl Icon for BurnLocked {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        "burn-locked".to_string()
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
            (model.mass(self.entity) / 1.0e22) as u64
        ]
    }

    fn position(&self, _view: &Scene, model: &Model) -> DVec2 {
        let burn = model.path_component(self.entity).last_segment_at_time(self.time).as_burn();
        model.absolute_position(burn.parent()) + burn.start_point().get_position()
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

        trace!("Locked burn icon clicked; switching to Selected");
        view.selected = Selected::Burn { entity: self.entity, time: self.time, state: BurnState::Selected }
    }
}