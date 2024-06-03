use eframe::egui::{Context, PointerState, Pos2};
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::{selected::Selected, util::compute_burn_arrow_position, Scene}};

use super::util::{burn_adjustment_amount, BurnAdjustDirection, BurnState};

fn compute_drag_adjustment_amount(view: &mut Scene, model: &Model, context: &Context, entity: Entity, time: f64, direction: BurnAdjustDirection, mouse_position: Pos2) -> DVec2 {
    let burn = model.burn_starting_at_time(entity, time);
    let burn_to_arrow_unit = burn.rotation_matrix() * direction.vector();
    let arrow_position = compute_burn_arrow_position(view, model, entity, time, direction);
    let arrow_to_mouse = view.camera.window_space_to_world_space(model, mouse_position, context.screen_rect()) - arrow_position;
    burn_adjustment_amount(arrow_to_mouse.dot(&burn_to_arrow_unit)) * direction.vector() * view.camera.zoom().powi(2)
}

pub fn update_adjustment(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, pointer: &PointerState) {
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

    // Do scroll adjustment
    if let Selected::Burn { entity, time, state: BurnState::Dragging(direction) } = view.selected.clone() {
        if let Some(mouse_position) = pointer.latest_pos() {
            let burn_time = model.timeline_event_at_time(entity, time).as_start_burn().unwrap().time();
            if let Some(amount) = model.calculate_burn_dv(entity, burn_time, compute_drag_adjustment_amount(view, model, context, entity, time, direction, mouse_position)) {
                events.push(Event::AdjustBurn { entity, time, amount });
            }
        }
    }
}