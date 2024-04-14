use eframe::egui::{PointerState, Rect};
use log::trace;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::Model;

use crate::{events::Event, game::{underlay::{icons::BURN_OFFSET, selected::Selected}, Scene}};

#[derive(Debug, Clone, PartialEq)]
pub enum BurnAdjustDirection {
    Prograde,
    Retrograde,
    Normal,
    Antinormal,
}

impl BurnAdjustDirection {
    pub fn get_vector(&self) -> DVec2 {
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
    Dragging(BurnAdjustDirection)
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
}

fn burn_adjustment_amount(amount: f64) -> f64 {
    if amount.is_sign_positive() {
        1.0e-8 * amount.powf(2.5)
    } else {
        1.0e3 * amount
    }
}

pub fn remove_if_expired(view: &mut Scene, model: &Model, time: f64) {
    if time < model.get_time() {
        trace!("Selected burn expired at time={time}");
        view.selected = Selected::None;
    }
}

pub fn update_selected(view: &mut Scene, model: &Model, events: &mut Vec<Event>, pointer: &PointerState, is_mouse_over_ui_element: bool, screen_rect: Rect) {
    // Deselected by clicking elsewhere
    if !is_mouse_over_ui_element && pointer.primary_clicked() {
        trace!("Selected burn deselected");
        view.selected = Selected::None;
    }
    
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

    // Do drag adjustment
    if let Selected::Burn { entity, time, state: BurnState::Dragging(direction) } = view.selected.clone() {
        if let Some(mouse_position) = pointer.latest_pos() {
            let burn = model.get_trajectory_component(entity).get_last_segment_at_time(time).as_burn();
            let burn_position = model.get_absolute_position(burn.get_parent()) + burn.get_start_point().get_position();
            let burn_to_arrow_unit = burn.get_rotation_matrix() * direction.get_vector();
            let relative_arrow_position = BURN_OFFSET * burn_to_arrow_unit / view.camera.get_zoom();
            let arrow_to_mouse = view.camera.window_space_to_world_space(model, mouse_position, screen_rect) - burn_position - relative_arrow_position;
            let amount = burn_adjustment_amount(arrow_to_mouse.dot(&burn_to_arrow_unit)) * direction.get_vector() * view.camera.get_zoom().powi(2);
            events.push(Event::AdjustBurn { entity, time, amount });
        }
    }
}