use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::components::vessel_component::faction::Faction;
use transfer_window_model::components::ComponentType;
use transfer_window_model::storage::entity_allocator::Entity;

use super::Icon;
use crate::game::events::ViewEvent;
use crate::game::selected::util::BurnState;
use crate::game::selected::Selected;
use crate::game::util::should_render_at_time;
use crate::game::View;

#[derive(Debug)]
pub struct Burn {
    entity: Entity,
    time: f64,
}

impl Burn {
    pub fn generate(view: &View) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.model.entities(vec![
            ComponentType::VesselComponent,
            ComponentType::PathComponent,
        ]) {
            let faction = view.model.vessel_component(entity).faction();
            if !Faction::Player.has_intel_for(faction) {
                continue;
            }
            for event in view.model.vessel_component(entity).timeline().events() {
                if event.is_start_burn() && should_render_at_time(view, entity, event.time()) {
                    let icon = Self {
                        entity,
                        time: event.time(),
                    };
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }
        icons
    }
}

impl Icon for Burn {
    fn texture(&self, _view: &View) -> String {
        "burn".to_string()
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
        16.0
    }

    fn priorities(&self, view: &View) -> [u64; 4] {
        [
            u64::from(self.is_selected(view)),
            0,
            5,
            view.model.mass(self.entity) as u64,
        ]
    }

    fn position(&self, view: &View) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Burn position");
        let burn = view.model.burn_starting_at_time(self.entity, self.time);
        view.model.absolute_position(burn.parent()) + burn.start_point().position()
    }

    fn facing(&self, _view: &View) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &View) -> bool {
        if let Selected::Burn {
            entity,
            time,
            state: _,
        } = &view.selected
        {
            *entity == self.entity && *time == self.time
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &View, pointer: &PointerState) {
        if !pointer.primary_clicked() {
            return;
        }

        if let Selected::Burn {
            entity,
            time,
            state,
        } = &view.selected
        {
            if *entity == self.entity
                && *time == self.time
                && view.model.timeline_event_at_time(self.entity, self.time).can_adjust(&view.model)
            {
                let state = if state.is_selected() {
                    trace!("Burn icon clicked; switching Selected -> Adjusting");
                    BurnState::Adjusting
                } else if state.is_adjusting() {
                    trace!("Burn icon clicked; switching Adjusting -> Selected");
                    BurnState::Selected
                } else {
                    // theoretically unreachable
                    state.clone()
                };
                let selected = Selected::Burn {
                    entity: *entity,
                    time: *time,
                    state,
                };
                view.add_view_event(ViewEvent::SetSelected(selected));
                return;
            }
        }

        trace!("Burn icon clicked; switching to Selected");
        let selected = Selected::Burn {
            entity: self.entity,
            time: self.time,
            state: BurnState::Selected,
        };
        view.add_view_event(ViewEvent::SetSelected(selected));
    }

    fn selectable(&self) -> bool {
        true
    }
}
