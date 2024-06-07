use log::trace;
use transfer_window_model::Model;

use super::{selected::Selected, util::{should_render, should_render_at_time}, Scene};


pub fn update(view: &mut Scene, model: &Model) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update expiry");

    // Unfocus camera if focus no longer exists
    if let Some(entity) = view.camera.focus() {
        if !model.entity_exists(entity) {
            view.camera.set_focus(None);
        }
    }

    // Remove selected if expired
    if let Some(time) = view.selected.time() {
        if time < model.time() {
            trace!("Selected expired at time={time}");
            view.selected = Selected::None;
        }
    }

    // Remove selected fire torpedo event if no longer exists
    if let Selected::FireTorpedo { entity, time, state: _ } = view.selected.clone() {
        if model.fire_torpedo_event_at_time(entity, time).is_none() {
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