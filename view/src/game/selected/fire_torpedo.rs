use eframe::egui::{PointerState, Pos2};
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::{events::{ModelEvent, ViewEvent}, selected::{util::BurnState, Selected}, util::compute_adjust_fire_torpedo_arrow_position, View};

use super::util::{burn_adjustment_amount, BurnAdjustDirection};

fn compute_drag_adjustment_amount(view: &View, entity: Entity, time: f64, direction: BurnAdjustDirection, mouse_position: Pos2) -> DVec2 {
    let event = view.model.fire_torpedo_event_at_time(entity, time).expect("No fire torpedo event found");
    let event_to_arrow_unit = view.model.snapshot_at(event.burn_time())
        .burn_starting_now(event.ghost())
        .rotation_matrix() * direction.vector();
    let arrow_position = compute_adjust_fire_torpedo_arrow_position(view, entity, time, direction);
    let arrow_to_mouse = view.window_space_to_world_space(mouse_position) - arrow_position;
    burn_adjustment_amount(arrow_to_mouse.dot(&event_to_arrow_unit)) * direction.vector() * view.camera.zoom().powi(2)
}

pub fn update_adjustment(view: &View, pointer: &PointerState) {
    // Finished dragging
    if let Selected::FireTorpedo { entity, time, state } = &view.selected {
        if state.is_dragging() {
            // We don't check primary_released() because this does not trigger if mouse is dragged outside of window
            if !pointer.primary_down() {
                trace!("Stopped dragging to adjust fire torpedo");
                let selected = Selected::FireTorpedo { entity: *entity, time: *time, state: BurnState::Adjusting };
                view.add_view_event(ViewEvent::SetSelected(selected));
            }
        }
    }

    // Do scroll adjustment
    if let Selected::FireTorpedo { entity, time, state: BurnState::Dragging(direction) } = view.selected.clone() {
        if let Some(mouse_position) = pointer.latest_pos() {
            let amount = compute_drag_adjustment_amount(view, entity, time, direction, mouse_position);
            view.add_model_event(ModelEvent::AdjustFireTorpedo { entity, time, amount });
        }
    }

}
