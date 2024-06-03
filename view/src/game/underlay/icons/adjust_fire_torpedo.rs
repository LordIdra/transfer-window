use eframe::egui::{PointerState, Rect};
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::{selected::{util::{BurnAdjustDirection, BurnState}, Selected}, util::compute_adjust_fire_torpedo_arrow_position, Scene};

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
    fn new(view: &mut Scene, model: &Model, entity: Entity, time: f64, direction: BurnAdjustDirection, pointer: &PointerState, screen_rect: Rect) -> Self {
        let event = model.fire_torpedo_event_at_time(entity, time).expect("No fire torpedo event found");
        let event_to_arrow_unit = model.burn_starting_at_time(event.ghost(), event.burn_time()).rotation_matrix() * direction.vector();
        let mut position = compute_adjust_fire_torpedo_arrow_position(view, model, entity, time, direction);

        // Additional offset if arrow is being dragged
        if let Some(mouse_position) = pointer.latest_pos() {
            if let Selected::FireTorpedo { entity: _, time: _, state: BurnState::Dragging(drag_direction) } = &view.selected {
                if *drag_direction == direction {
                    let arrow_to_mouse = view.camera.window_space_to_world_space(model, mouse_position, screen_rect) - position;
                    let amount = arrow_to_mouse.dot(&event_to_arrow_unit);
                    position += offset(amount) * event_to_arrow_unit;
                }
            }
        }

        Self { entity, time, position, direction }
    }

    pub fn generate(view: &mut Scene, model: &Model, pointer: &PointerState, screen_rect: Rect) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        if let Selected::FireTorpedo { entity, time, state } = view.selected.clone() {
            if state.is_adjusting() || state.is_dragging() {
                let icon = Self::new(view, model, entity, time, BurnAdjustDirection::Prograde, pointer, screen_rect);
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self::new(view, model, entity, time, BurnAdjustDirection::Retrograde, pointer, screen_rect);
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self::new(view, model, entity, time, BurnAdjustDirection::Normal, pointer, screen_rect);
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self::new(view, model, entity, time, BurnAdjustDirection::Antinormal, pointer, screen_rect);
                icons.push(Box::new(icon) as Box<dyn Icon>);
            }
        }
        icons
    }
}

impl Icon for AdjustFireTorpedo {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        "adjust-burn-arrow".to_string()
    }

    fn alpha(&self, view: &Scene, _model: &Model, _is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
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

    fn radius(&self, _view: &Scene, _model: &Model) -> f64 {
        16.0
    }

    fn priorities(&self, _view: &Scene, _model: &Model) -> [u64; 4] {
        [3, 0, 0, 0]
    }

    fn position(&self, _view: &Scene, _model: &Model, ) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Adjust fire torpedo position");
        self.position
    }

    fn facing(&self, _view: &Scene, model: &Model) -> Option<DVec2> {
        let event = model.fire_torpedo_event_at_time(self.entity, self.time).expect("No fire torpedo event found");
        Some(model.burn_starting_at_time(event.ghost(), event.burn_time()).rotation_matrix() * self.direction.vector())
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        if let Selected::FireTorpedo { entity: _, time: _, state: BurnState::Dragging(direction) } = &view.selected {
            return *direction == self.direction
        }
        false
    }

    fn on_mouse_over(&self, view: &mut Scene, _model: &Model, pointer: &PointerState) {
        if let Selected::FireTorpedo { entity: _, time: _, state } = &mut view.selected {
            if pointer.primary_clicked() {
                trace!("Started dragging to adjust fire torpedo {:?}", self.direction);
                *state = BurnState::Dragging(self.direction);
            }
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}
