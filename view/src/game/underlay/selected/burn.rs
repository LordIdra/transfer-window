use eframe::egui::{Context, PointerState, Pos2, Vec2};
use log::trace;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::{underlay::selected::Selected, util::compute_burn_arrow_position, Scene}};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BurnAdjustDirection {
    Prograde,
    Retrograde,
    Normal,
    Antinormal,
}

impl BurnAdjustDirection {
    pub fn vector(self) -> DVec2 {
        match self {
            BurnAdjustDirection::Prograde => vec2(1.0, 0.0),
            BurnAdjustDirection::Retrograde => vec2(-1.0, 0.0),
            BurnAdjustDirection::Normal => vec2(0.0, 1.0),
            BurnAdjustDirection::Antinormal => vec2(0.0, -1.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BurnState {
    Selected,
    Adjusting,
    Dragging(BurnAdjustDirection),
    Scrolling(BurnAdjustDirection, Vec2)
}

impl BurnState {
    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected)
    }

    pub fn is_adjusting(&self) -> bool {
        matches!(self, Self::Adjusting)
    }

    pub fn is_dragging(&self) -> bool {
        matches!(self, Self::Dragging(_))
    }

    pub fn is_scrolling(&self) -> bool {
        matches!(self, Self::Scrolling(_, _))
    }
}

fn burn_adjustment_amount(amount: f64) -> f64 {
    if amount.is_sign_positive() {
        1.0e-8 * amount.powf(2.5)
    } else {
        1.0e3 * amount
    }
}

fn compute_drag_adjustment_amount(view: &mut Scene, model: &Model, context: &Context, entity: Entity, time: f64, direction: BurnAdjustDirection, mouse_position: Pos2) -> DVec2 {
    let burn = model.burn_starting_at_time(entity, time);
    let burn_to_arrow_unit = burn.rotation_matrix() * direction.vector();
    let arrow_position = compute_burn_arrow_position(view, model, entity, time, direction);
    let arrow_to_mouse = view.camera.window_space_to_world_space(model, mouse_position, context.screen_rect()) - arrow_position;
    burn_adjustment_amount(arrow_to_mouse.dot(&burn_to_arrow_unit)) * direction.vector() * view.camera.zoom().powi(2)
}

pub fn update_drag(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, pointer: &PointerState) {
    // Finished dragging
    if let Selected::Burn { entity: _, time: _, state } = &mut view.selected {
        if state.is_dragging() {
            // We don't check primary_released() because this does not trigger if mouse is dragged outside of window
            if !pointer.primary_down() {
                trace!("Stopped dragging to adjust burn");
                *state = BurnState::Adjusting;
            }
        }
    }

    // Do adjustment
    if let Selected::Burn { entity, time, state } = view.selected.clone() {
        match state {
            BurnState::Dragging(direction) => {
                if let Some(mouse_position) = pointer.latest_pos() {
                    let amount = compute_drag_adjustment_amount(view, model, context, entity, time, direction, mouse_position);
                    events.push(Event::AdjustBurn { entity, time, amount });
                }
            }
            // Inspired by KSP-type scrolling
            BurnState::Scrolling(direction, scroll_delta) => {
                // I'm quite sure there's some mathematical operation for this,
                // but I don't know what it is
                let aligned = match direction {
                    BurnAdjustDirection::Prograde => vec2(-scroll_delta.y as f64, scroll_delta.x as f64),
                    BurnAdjustDirection::Retrograde => vec2(scroll_delta.y as f64, scroll_delta.x as f64),
                    BurnAdjustDirection::Normal => vec2(scroll_delta.x as f64, -scroll_delta.y as f64),
                    BurnAdjustDirection::Antinormal => vec2(scroll_delta.x as f64, scroll_delta.y as f64),
                };
                let length = aligned.norm();
                let amount_scale = burn_adjustment_amount(length) / length;
                let amount = aligned * amount_scale * 1e3;
                events.push(Event::AdjustBurn { entity, time, amount });
            }
            _ => {}
        }
    }

    if let Selected::Burn { entity: _, time: _, state } = &mut view.selected {
        if state.is_scrolling() {
            trace!("Stopped scrolling to adjust burn");
            *state = BurnState::Adjusting;
        }
    }
}