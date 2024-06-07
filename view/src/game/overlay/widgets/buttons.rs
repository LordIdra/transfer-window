use eframe::egui::{Context, Ui};
use transfer_window_model::{components::vessel_component::timeline::{enable_guidance::EnableGuidanceEvent, start_burn::StartBurnEvent}, storage::entity_allocator::Entity, Model};

use crate::game::Scene;

use super::custom_image_button::CustomCircularImageButton;

// Returns new time if could be drawn & clicked
pub fn draw_previous(view: &Scene, model: &Model, context: &Context, ui: &mut Ui, time: f64, entity: Entity) -> Option<f64> {
    let orbit = model.orbit_at_time(entity, time);
    let time = time - orbit.period()?;
    let enabled = time > orbit.current_point().time();
    let button = CustomCircularImageButton::new(view, "previous-orbit", context.screen_rect(), 36.0)
        .with_enabled(enabled)
        .with_padding(10.0);
    if ui.add_enabled(enabled, button).on_hover_text("Previous orbit").clicked() {
        Some(time)
    } else {
        None
    }
}

// Returns new time if could be drawn & clicked
pub fn draw_next(view: &Scene, model: &Model, context: &Context, ui: &mut Ui, time: f64, entity: Entity) -> Option<f64> {
    let orbit = model.orbit_at_time(entity, time);
    let time = time + orbit.period()?;
    let enabled = time < orbit.end_point().time();
    let button = CustomCircularImageButton::new(view, "next-orbit", context.screen_rect(), 36.0)
        .with_enabled(enabled)
        .with_padding(10.0);
    if ui.add_enabled(enabled, button).on_hover_text("Next orbit").clicked() {
        Some(time)
    } else {
        None
    }
}

/// Returns true if was clicked
pub fn draw_warp_to(view: &Scene, model: &Model, context: &Context, ui: &mut Ui, time: f64) -> bool {
    let enabled = model.can_warp_to(time);
    let button = CustomCircularImageButton::new(view, "warp-here", context.screen_rect(), 36.0)
        .with_enabled(enabled)
        .with_padding(8.0);
    ui.add_enabled(enabled, button).on_hover_text("Warp here").clicked()
}

/// Returns true if could create and was clicked
pub fn draw_create_burn(view: &Scene, model: &Model, context: &Context, ui: &mut Ui, entity: Entity, time: f64) -> bool {
    if !StartBurnEvent::can_create_ever(model, entity) {
        return false;
    }
    let enabled = StartBurnEvent::can_create(model, entity, time);
    let button = CustomCircularImageButton::new(view, "create-burn", context.screen_rect(), 36.0)
        .with_enabled(enabled)
        .with_padding(5.0);
    ui.add_enabled(enabled, button).on_hover_text("Create burn").clicked()
}

/// Returns true if could create and was clicked
pub fn draw_enable_guidance(view: &Scene, model: &Model, context: &Context, ui: &mut Ui, entity: Entity, time: f64) -> bool {
    if !EnableGuidanceEvent::can_create_ever(model, entity) {
        return false;
    }
    let enabled = EnableGuidanceEvent::can_create(model, entity, time);
    let button = CustomCircularImageButton::new(view, "enable-guidance", context.screen_rect(), 36.0)
        .with_enabled(enabled)
        .with_padding(5.0);
    ui.add_enabled(enabled, button).on_hover_text("Enable guidance").clicked()
}

pub fn draw_edit_vessel(view: &Scene, model: &Model, context: &Context, ui: &mut Ui, entity: Entity) -> bool {
    let enabled = model.can_edit(entity);
    let button = CustomCircularImageButton::new(view, "edit", context.screen_rect(), 36.0)
        .with_padding(8.0)
        .with_enabled(enabled);
    ui.add_enabled(enabled, button).on_hover_text("Edit").clicked()
}

pub fn draw_cancel_burn(view: &Scene, context: &Context, ui: &mut Ui) -> bool {
    let button = CustomCircularImageButton::new(view, "cancel", context.screen_rect(), 36.0)
        .with_padding(8.0);
    ui.add(button).on_hover_text("Cancel current burn").clicked()
}

pub fn draw_cancel_guidance(view: &Scene, context: &Context, ui: &mut Ui) -> bool {
    let button = CustomCircularImageButton::new(view, "cancel", context.screen_rect(), 36.0)
        .with_padding(8.0);
    ui.add(button).on_hover_text("Cancel current guidance").clicked()
}