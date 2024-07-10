use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::components::vessel_component::faction::Faction;
use transfer_window_model::storage::entity_allocator::Entity;

use super::Icon;
use crate::game::events::ViewEvent;
use crate::game::selected::Selected;
use crate::game::util::{should_render_at_time, ApproachType};
use crate::game::View;

#[derive(Debug)]
pub struct ClosestApproach {
    type_: ApproachType,
    vessel: Entity, // the vessel that is targeting another vessel
    target: Entity,
    time: f64,
    position: DVec2,
}

impl ClosestApproach {
    pub fn new(
        view: &View,
        type_: ApproachType,
        vessel: Entity,
        target: Entity,
        entity: Entity,
        time: f64,
    ) -> Self {
        let orbit = view.model.orbit_at_time(entity, time, Some(Faction::Player));
        let position =
            view.model.absolute_position(orbit.parent()) + orbit.point_at_time(time).position();
        Self {
            type_,
            vessel,
            target,
            time,
            position,
        }
    }

    pub fn generate(view: &View) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        if let Some(entity) = view.selected.entity(&view.model) {
            if let Some(vessel_component) = view.model.try_vessel_component(entity) {
                if !Faction::Player.has_intel_for(vessel_component.faction()) {
                    return vec![];
                }
                if let Some(target) = vessel_component.target() {
                    let (approach_1, approach_2) = view.model.find_next_two_closest_approaches(
                        entity,
                        target,
                        Some(Faction::Player),
                    );

                    if let Some(time) = approach_1 {
                        if should_render_at_time(view, entity, time) {
                            let icon =
                                Self::new(view, ApproachType::First, entity, target, entity, time);
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                            let icon =
                                Self::new(view, ApproachType::First, entity, target, target, time);
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                        }
                    }

                    if let Some(time) = approach_2 {
                        if should_render_at_time(view, entity, time) {
                            let icon =
                                Self::new(view, ApproachType::Second, entity, target, entity, time);
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                            let icon =
                                Self::new(view, ApproachType::Second, entity, target, target, time);
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
    fn texture(&self, _view: &View) -> String {
        match self.type_ {
            ApproachType::First => "closest-approach-1",
            ApproachType::Second => "closest-approach-2",
        }
        .to_string()
    }

    fn alpha(&self, _view: &View, is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            return 0.4;
        }
        if is_selected {
            return 1.0;
        }
        if is_hovered {
            return 0.8;
        }
        0.6
    }

    fn radius(&self, _view: &View) -> f64 {
        8.0
    }

    fn priorities(&self, view: &View) -> [u64; 4] {
        [u64::from(self.is_selected(view)), 0, 3, 0]
    }

    fn position(&self, _view: &View) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Closest approach position");
        self.position
    }

    fn facing(&self, _view: &View) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &View) -> bool {
        if let Selected::Approach {
            type_,
            entity,
            target: _,
            time: _,
        } = &view.selected
        {
            // time can very slightly change as approach is recalculated so not a reliable
            // way to determine equality
            *type_ == self.type_ && *entity == self.vessel
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &View, pointer: &PointerState) {
        if pointer.primary_clicked() {
            trace!("Approach icon clicked; switching to Selected");
            let selected = Selected::Approach {
                type_: self.type_,
                entity: self.vessel,
                target: self.target,
                time: self.time,
            };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}
