use eframe::egui::{PointerState, Rect, Vec2};
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::{util::{BurnAdjustDirection, BurnState}, Selected}, util::compute_burn_arrow_position, Scene};

use super::Icon;

fn offset(amount: f64) -> f64 {
    if amount.is_sign_positive() {
        0.2 * amount
    } else {
        0.02 * amount
    }
}

#[derive(Debug)]
pub struct AdjustBurn {
    entity: Entity,
    time: f64,
    position: DVec2,
    direction: BurnAdjustDirection,
}

impl AdjustBurn {
    fn new(view: &Scene, model: &Model, entity: Entity, time: f64, direction: BurnAdjustDirection, pointer: &PointerState, screen_rect: Rect) -> Self {
        let burn = model.burn_starting_at_time(entity, time);
        let burn_to_arrow_unit = burn.rotation_matrix() * direction.vector();
        let mut position = compute_burn_arrow_position(view, model, entity, time, direction);

        // Additional offset if arrow is being dragged
        if let Some(mouse_position) = pointer.latest_pos() {
            if let Selected::Burn { entity: _, time: _, state: BurnState::Dragging(drag_direction) } = &view.selected {
                if *drag_direction == direction {
                    let arrow_to_mouse = view.camera.window_space_to_world_space(model, mouse_position, screen_rect) - position;
                    let amount = arrow_to_mouse.dot(&burn_to_arrow_unit);
                    position += offset(amount) * burn_to_arrow_unit;
                }
            }
        }

        Self { entity, time, position, direction }
    }

    pub fn generate(view: &Scene, model: &Model, pointer: &PointerState, screen_rect: Rect) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        if let Selected::Burn { entity, time, state } = view.selected.clone() {
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

impl Icon for AdjustBurn {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        "adjust-burn-arrow".to_string()
    }

    fn alpha(&self, view: &Scene, _model: &Model, _is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            return 0.0;
        }
        if let Selected::Burn { entity: _, time: _, state: BurnState::Dragging(direction) } = &view.selected {
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
        10.0
    }

    fn priorities(&self, _view: &Scene, _model: &Model) -> [u64; 4] {
        [2, 0, 0, 0]
    }

    fn position(&self, _view: &Scene, _model: &Model, ) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Adjust burn position");
        self.position
    }

    fn facing(&self, _view: &Scene, model: &Model) -> Option<DVec2> {
        let burn = model.burn_starting_at_time(self.entity, self.time);
        Some(burn.rotation_matrix() * self.direction.vector())
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        if let Selected::Burn { entity: _, time: _, state: BurnState::Dragging(direction) } = &view.selected {
            return *direction == self.direction
        }
        false
    }

    fn on_mouse_over(&self, view: &mut Scene, _model: &Model, pointer: &PointerState) {
        if let Selected::Burn { entity: _, time: _, state } = &mut view.selected {
            if pointer.primary_clicked() {
                trace!("Started dragging to adjust burn {:?}", self.direction);
                *state = BurnState::Dragging(self.direction);
            }
        }
    }

    fn on_scroll(&self, view: &mut Scene, _model: &Model, scroll_delta: Vec2) -> bool {
        if let Selected::Burn { entity: _, time: _, state } = &mut view.selected {
            trace!("Scrolled to adjust burn {:?}", self.direction);
            *state = BurnState::Scrolling(self.direction, scroll_delta);
            true
        } else {
            false
        }
    }
}
