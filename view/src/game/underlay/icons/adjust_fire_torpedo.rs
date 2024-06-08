use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::{events::ViewEvent, selected::{util::{BurnAdjustDirection, BurnState}, Selected}, util::compute_adjust_fire_torpedo_arrow_position, View};

use super::Icon;

fn offset(amount: f64) -> f64 {
    if amount.is_sign_positive() {
        0.2 * amount
    } else {
        0.02 * amount
    }
}

#[derive(Debug)]
pub struct AdjustFireTorpedo {
    entity: Entity,
    time: f64,
    position: DVec2,
    direction: BurnAdjustDirection,
}

impl AdjustFireTorpedo {
    fn new(view: &View, entity: Entity, time: f64, direction: BurnAdjustDirection, pointer: &PointerState) -> Self {
        let event = view.model.fire_torpedo_event_at_time(entity, time).expect("No fire torpedo event found");
        let event_to_arrow_unit = view.model.burn_starting_at_time(event.ghost(), event.burn_time()).rotation_matrix() * direction.vector();
        let mut position = compute_adjust_fire_torpedo_arrow_position(view, entity, time, direction);

        // Additional offset if arrow is being dragged
        if let Some(mouse_position) = pointer.latest_pos() {
            if let Selected::FireTorpedo { entity: _, time: _, state: BurnState::Dragging(drag_direction) } = &view.selected {
                if *drag_direction == direction {
                    let arrow_to_mouse = view.window_space_to_world_space(mouse_position) - position;
                    let amount = arrow_to_mouse.dot(&event_to_arrow_unit);
                    position += offset(amount) * event_to_arrow_unit;
                }
            }
        }

        Self { entity, time, position, direction }
    }

    pub fn generate(view: &View, pointer: &PointerState) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        if let Selected::FireTorpedo { entity, time, state } = view.selected.clone() {
            if state.is_adjusting() || state.is_dragging() {
                let icon = Self::new(view, entity, time, BurnAdjustDirection::Prograde, pointer);
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self::new(view, entity, time, BurnAdjustDirection::Retrograde, pointer);
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self::new(view, entity, time, BurnAdjustDirection::Normal, pointer);
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self::new(view, entity, time, BurnAdjustDirection::Antinormal, pointer);
                icons.push(Box::new(icon) as Box<dyn Icon>);
            }
        }
        icons
    }
}

impl Icon for AdjustFireTorpedo {
    fn texture(&self, _view: &View) -> String {
        "adjust-burn-arrow".to_string()
    }

    fn alpha(&self, view: &View, __is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            return 0.0;
        }
        if let Selected::FireTorpedo { entity: _, time: _, state: BurnState::Dragging(direction) } = &view.selected {
            if *direction == self.direction {
                return 1.0;
            }
            // Dim the other arrows if we are dragging one
            return 0.4;
        }
        if is_hovered {
            0.8
        } else {
            0.6
        }
    }

    fn radius(&self, _view: &View) -> f64 {
        16.0
    }

    fn priorities(&self, _view: &View) -> [u64; 4] {
        [3, 0, 0, 0]
    }

    fn position(&self, _view: &View) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Adjust fire torpedo position");
        self.position
    }

    fn facing(&self, view: &View) -> Option<DVec2> {
        let event = view.model.fire_torpedo_event_at_time(self.entity, self.time).expect("No fire torpedo event found");
        Some(view.model.burn_starting_at_time(event.ghost(), event.burn_time()).rotation_matrix() * self.direction.vector())
    }

    fn is_selected(&self, view: &View) -> bool {
        if let Selected::FireTorpedo { entity: _, time: _, state: BurnState::Dragging(direction) } = &view.selected {
            return *direction == self.direction
        }
        false
    }

    fn on_mouse_over(&self, view: &View, pointer: &PointerState) {
        if let Selected::FireTorpedo { entity, time, state: _ } = &view.selected {
            if pointer.primary_down() {
                trace!("Started dragging to adjust fire torpedo {:?}", self.direction);
                let state = BurnState::Dragging(self.direction);
                let selected = Selected::FireTorpedo { entity: *entity, time: *time, state };
                view.add_view_event(ViewEvent::SetSelected(selected));
            }
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}
