use log::trace;
use transfer_window_model::Model;

use super::{underlay::selected::Selected, util::{should_render, should_render_at_time}, Scene};


pub fn update(view: &mut Scene, model: &Model) {
    // Unfocus camera if focus no longer exists
    if let Some(entity) = view.camera.focus() {
        if !model.entity_exists(entity) {
            view.camera.set_focus(None);
        }
    }

    // Remove selected point if expired
    if let Selected::Point { entity: _, time } = view.selected.clone() {
        if time < model.time() {
            trace!("Selected segment point expired at time={time}");
            view.selected = Selected::None;
        }
    }

    // Remove selected burn if expired
    if let Selected::Burn { entity: _, time, state: _ } = view.selected.clone() {
        if time < model.time() {
            trace!("Selected burn expired at time={time}");
            view.selected = Selected::None;
        }
    }

    // Remove selected fire torpedo event if expired
    if let Selected::FireTorpedo { entity, time, state: _ } = view.selected.clone() {
        if model.fire_torpedo_event_at_time(entity, time).is_none() || time < model.time() {
            trace!("Selected fire torpedo event expired at time={time}");
            view.selected = Selected::None;
        }
    }

    // Remove selected guidance event if expired
    if let Selected::EnableGuidance { entity: _, time } = view.selected.clone() {
        if time < model.time() {
            trace!("Selected fire torpedo event expired at time={time}");
            view.selected = Selected::None;
        }
    }

    // Delete selected if its entity no longer exists or should not be rendered
    if let Some(entity) = view.selected.entity(model) {
        if !model.entity_exists(entity) {
            view.selected = Selected::None;
        }
    }

    // Deleted selected if its entity should not be rendered
    if let Some(entity) = view.selected.entity(model) {
        let should_render = match view.selected.time() {
            Some(time) => should_render_at_time(view, model, entity, time),
            None => should_render(view, model, entity),
        };
        if !should_render {
            view.selected = Selected::None;
        }
    }
}