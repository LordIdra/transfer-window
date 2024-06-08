use log::trace;

use super::{selected::Selected, util::{should_render, should_render_at_time}, View};


pub fn update(view: &mut View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update expiry");

    // Unfocus camera if focus no longer exists
    if let Some(entity) = view.camera.focus() {
        if !view.model.entity_exists(entity) {
            view.camera.set_focus(None);
        }
    }

    // Remove selected if expired
    if let Some(time) = view.selected.time() {
        if time < view.model.time() {
            trace!("Selected expired at time={time}");
            view.selected = Selected::None;
        }
    }

    // Remove selected approach if target is no longer targeted
    if let Selected::Approach { type_: _, entity, target, time: _ } = view.selected.clone() {
        if !view.model.vessel_component(entity).has_target() || view.model.vessel_component(entity).target().unwrap() != target {
            trace!("Selected approach no longer has target");
            view.selected = Selected::None;
        }
    }

    // Remove selected fire torpedo event if no longer exists
    if let Selected::FireTorpedo { entity, time, state: _ } = view.selected.clone() {
        if view.model.fire_torpedo_event_at_time(entity, time).is_none() {
            trace!("Selected fire torpedo event expired at time={time}");
            view.selected = Selected::None;
        }
    }

    // Delete selected if its entity no longer exists or should not be rendered
    if let Some(entity) = view.selected.entity(&view.model) {
        if !view.model.entity_exists(entity) {
            view.selected = Selected::None;
        }
    }

    // Deleted selected if its entity should not be rendered
    if let Some(entity) = view.selected.entity(&view.model) {
        let should_render = match view.selected.time() {
            Some(time) => should_render_at_time(view, entity, time),
            None => should_render(view, entity),
        };
        if !should_render {
            view.selected = Selected::None;
        }
    }
}