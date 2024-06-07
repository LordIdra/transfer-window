use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::{selected::Selected, util::{should_render_at_time, ApproachType}, Scene};

use super::Icon;

#[derive(Debug)]
pub struct ClosestApproach {
    type_: ApproachType,
    vessel: Entity, // the vessel that is targeting another vessel
    target: Entity,
    entity: Entity, // which entity to draw this approach icon for
    time: f64,
}

impl ClosestApproach {
    pub fn generate(view: &Scene, model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        if let Some(entity) = view.selected.entity(model) {
            if let Some(vessel_component) = model.try_vessel_component(entity) {
                if let Some(target) = vessel_component.target() {
                    let (approach_1, approach_2) = model.find_next_two_closest_approaches(entity, target);
                    
                    if let Some(time) = approach_1 {
                        if should_render_at_time(view, model, entity, time) {
                            let icon = Self { type_: ApproachType::First, vessel: entity, target, entity, time };
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                            let icon = Self { type_: ApproachType::First, vessel: entity, target, entity: target, time };
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                        }
                    }

                    if let Some(time) = approach_2 {
                        if should_render_at_time(view, model, entity, time) {
                            let icon = Self { type_: ApproachType::Second, vessel: entity, target, entity, time };
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                            let icon = Self { type_: ApproachType::Second, vessel: entity, target, entity: target, time };
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                        }
                    }
                }
            }
        }
        icons
    }
}

impl Icon for ClosestApproach {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        match self.type_ {
            ApproachType::First => "closest-approach-1",
            ApproachType::Second => "closest-approach-2",
        }.to_string()
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
            3,
            0,
        ]
    }

    fn position(&self, _view: &Scene, model: &Model) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Closest approach position");
        let orbit = model.orbit_at_time(self.entity, self.time);
        model.absolute_position(orbit.parent()) + orbit.point_at_time(self.time).position()
    }

    fn facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        if let Selected::Approach { type_, entity, target: _, time: _ } = &view.selected {
            // time can very slightly change as approach is recalculated so not a reliable way to determine equality
            *type_ == self.type_ && *entity == self.vessel
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &mut Scene, _model: &Model, pointer: &PointerState) {
        if pointer.primary_clicked() {
            trace!("Approach icon clicked; switching to Selected");
            view.selected = Selected::Approach { type_: self.type_, entity: self.vessel, target: self.target, time: self.time };
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}