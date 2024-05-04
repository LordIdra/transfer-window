use log::trace;
use transfer_window_model::Model;

use super::{underlay::selected::Selected, Scene};


pub fn update(view: &mut Scene, model: &Model) {
    // Unfocus camera if focus no longer exists
    if let Some(entity) = view.camera.focus() {
        if !model.entity_exists(entity) {
            view.camera.set_focus(None);
        }
    }
    
    // Delete selected if its entity no longer exists
    if let Some(entity) = view.selected.selected_entity() {
        if !model.entity_exists(entity) {
            view.selected = Selected::None;
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

    // Delete current menu if the entity no longer exists
    if let Some(entity) = view.right_click_menu {
        if !model.entity_exists(entity) {
            view.right_click_menu = None;
        }
    }

    // Delete current vessel editor if the entity no longer exists
    if let Some(vessel_editor) = view.vessel_editor.as_ref() {
        if !model.entity_exists(vessel_editor.entity()) {
            view.vessel_editor = None;
        }
    }
}