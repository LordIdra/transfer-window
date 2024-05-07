use eframe::egui::{Context, PointerState, Pos2};
use log::trace;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::{underlay::selected::{util::BurnState, Selected}, util::compute_adjust_fire_torpedo_arrow_position, Scene}};

use super::util::{burn_adjustment_amount, BurnAdjustDirection};

fn compute_drag_adjustment_amount(view: &mut Scene, model: &Model, context: &Context, entity: Entity, time: f64, direction: BurnAdjustDirection, mouse_position: Pos2) -> DVec2 {
    let event = model.fire_torpedo_event_at_time(entity, time).expect("No fire torpedo event found");
    let event_to_arrow_unit = model.burn_starting_at_time(event.ghost(), event.burn_time()).rotation_matrix() * direction.vector();
    let arrow_position = compute_adjust_fire_torpedo_arrow_position(view, model, entity, time, direction);
    let arrow_to_mouse = view.camera.window_space_to_world_space(model, mouse_position, context.screen_rect()) - arrow_position;
    burn_adjustment_amount(arrow_to_mouse.dot(&event_to_arrow_unit)) * direction.vector() * view.camera.zoom().powi(2)
}

pub fn update_adjustment(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, pointer: &PointerState) {
    // Finished dragging
    if let Selected::FireTorpedo { entity: _, time: _, state } = &mut view.selected {
        if state.is_dragging() {
            // We don't check primary_released() because this does not trigger if mouse is dragged outside of window
            if !pointer.primary_down() {
                trace!("Stopped dragging to adjust fire torpedo");
                *state = BurnState::Adjusting;
            }
        }
    }

    // Do scroll adjustment
    if let Selected::FireTorpedo { entity, time, state } = view.selected.clone() {
        match state {
            BurnState::Dragging(direction) => {
                if let Some(mouse_position) = pointer.latest_pos() {
                    let amount = compute_drag_adjustment_amount(view, model, context, entity, time, direction, mouse_position);
                    events.push(Event::AdjustFireTorpedo { entity, time, amount });
                }
            }
            // Inspired by KSP-type scrolling
            BurnState::Scrolling(direction, scroll_delta) => {
                // Use this instead of direction.vector() because it's honestly just simpler like this
                let aligned = match direction {
                    BurnAdjustDirection::Prograde => vec2(-scroll_delta.y as f64, scroll_delta.x as f64),
                    BurnAdjustDirection::Retrograde => vec2(scroll_delta.y as f64, scroll_delta.x as f64),
                    BurnAdjustDirection::Normal => vec2(scroll_delta.x as f64, -scroll_delta.y as f64),
                    BurnAdjustDirection::Antinormal => vec2(scroll_delta.x as f64, scroll_delta.y as f64),
                };
                let length = aligned.norm();
                let amount_scale = burn_adjustment_amount(length) / length;
                let amount = aligned * amount_scale * 1e3;
                events.push(Event::AdjustFireTorpedo { entity, time, amount });
            }
            _ => {}
        }
    }

    if let Selected::FireTorpedo { entity: _, time: _, state } = &mut view.selected {
        if state.is_scrolling() {
            *state = BurnState::Adjusting;
        }
    }
}