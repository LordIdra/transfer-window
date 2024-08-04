use eframe::egui::Ui;
use transfer_window_model::{components::{orbitable_component::OrbitableType, vessel_component::{faction::Faction, timeline::{enable_guidance::EnableGuidanceEvent, fire_torpedo::FireTorpedoEvent, start_burn::StartBurnEvent}}}, storage::entity_allocator::Entity};

use crate::game::{util::{orbitable_texture, vessel_texture}, View};

use super::custom_image_button::CustomCircularImageButton;

pub fn draw_select_vessel(view: &View, ui: &mut Ui, entity: Entity) -> bool {
    let vessel_component = view.model.vessel_component(entity);
    let icon = vessel_texture(vessel_component);
    let tooltip = "Select ".to_string() + vessel_component.class().name();
    let button = CustomCircularImageButton::new(view, icon, 36.0)
        .with_padding(8.0);
    ui.add(button).on_hover_text(tooltip).clicked()
}

pub fn draw_select_orbitable(view: &View, ui: &mut Ui, entity: Entity) -> bool {
    let type_ = view.model.orbitable_component(entity).type_();
    let icon = orbitable_texture(type_);
    let tooltip = match type_ {
        OrbitableType::Star => "Select star",
        OrbitableType::Planet => "Select planet",
        OrbitableType::Moon => "Select moon",
    };
    let button = CustomCircularImageButton::new(view, icon, 36.0)
        .with_padding(8.0);
    ui.add(button).on_hover_text(tooltip).clicked()
}

// Returns new time if could be drawn & clicked
pub fn draw_previous(view: &View, ui: &mut Ui, time: f64, entity: Entity) -> Option<f64> {
    let orbit = view.model.orbit_at_time(entity, time, Some(Faction::Player));
    let time = time - orbit.period()?;
    let enabled = time > orbit.current_point().time();
    let button = CustomCircularImageButton::new(view, "previous-orbit", 36.0)
        .with_enabled(enabled)
        .with_padding(10.0);
    if ui.add_enabled(enabled, button).on_hover_text("Previous orbit").clicked() {
        Some(time)
    } else {
        None
    }
}

// Returns new time if could be drawn & clicked
pub fn draw_next(view: &View, ui: &mut Ui, time: f64, entity: Entity) -> Option<f64> {
    let orbit = view.model.orbit_at_time(entity, time, Some(Faction::Player));
    let time = time + orbit.period()?;
    let enabled = time < orbit.end_point().time();
    let button = CustomCircularImageButton::new(view, "next-orbit", 36.0)
        .with_enabled(enabled)
        .with_padding(10.0);
    if ui.add_enabled(enabled, button).on_hover_text("Next orbit").clicked() {
        Some(time)
    } else {
        None
    }
}

/// Returns true if was clicked
pub fn draw_warp_to(view: &View, ui: &mut Ui, time: f64) -> bool {
    let enabled = view.model.can_warp_to(time);
    let button = CustomCircularImageButton::new(view, "warp-here", 36.0)
        .with_enabled(enabled)
        .with_padding(8.0);
    ui.add_enabled(enabled, button).on_hover_text("Warp here").clicked()
}

/// Returns true if could create and was clicked
pub fn draw_create_burn(view: &View, ui: &mut Ui, entity: Entity, time: f64) -> bool {
    if !StartBurnEvent::can_create_ever(&view.model, entity) {
        return false;
    }
    let enabled = StartBurnEvent::can_create(&view.model, entity, time);
    let button = CustomCircularImageButton::new(view, "create-burn", 36.0)
        .with_enabled(enabled)
        .with_padding(5.0);
    ui.add_enabled(enabled, button).on_hover_text("Create burn").clicked()
}

/// Returns true if could create and was clicked
pub fn draw_enable_guidance(view: &View, ui: &mut Ui, entity: Entity, time: f64) -> bool {
    if !EnableGuidanceEvent::can_create_ever(&view.model, entity) {
        return false;
    }
    let enabled = EnableGuidanceEvent::can_create(&view.model, entity, time);
    let button = CustomCircularImageButton::new(view, "enable-guidance", 36.0)
        .with_enabled(enabled)
        .with_padding(5.0);
    ui.add_enabled(enabled, button).on_hover_text("Enable guidance").clicked()
}

pub fn draw_fire_torpedo(view: &View, ui: &mut Ui, entity: Entity, time: f64) -> bool {
    if !FireTorpedoEvent::can_create_ever(&view.model, entity) {
        return false;
    }
    let enabled = FireTorpedoEvent::can_create(&view.model, entity, time);
    let button = CustomCircularImageButton::new(view, "fire-torpedo", 36.0)
        .with_enabled(enabled)
        .with_padding(5.0);
    ui.add_enabled(enabled, button).on_hover_text("Fire torpedo").clicked()
}

pub fn draw_cancel_burn(view: &View, ui: &mut Ui) -> bool {
    let button = CustomCircularImageButton::new(view, "cancel", 36.0)
        .with_padding(8.0);
    ui.add(button).on_hover_text("Cancel burn").clicked()
}

pub fn draw_cancel_guidance(view: &View, ui: &mut Ui) -> bool {
    let button = CustomCircularImageButton::new(view, "cancel", 36.0)
        .with_padding(8.0);
    ui.add(button).on_hover_text("Cancel guidance").clicked()
}

pub fn draw_focus(view: &View, ui: &mut Ui) -> bool {
    let button = CustomCircularImageButton::new(view, "focus", 36.0)
        .with_padding(8.0);
    ui.add(button).on_hover_text("Focus").clicked()
}

pub fn draw_dock(view: &View, ui: &mut Ui, entity: Entity) -> bool {
    let enabled = view.model.can_dock(entity);
    let button = CustomCircularImageButton::new(view, "dock", 36.0)
        .with_enabled(enabled)
        .with_padding(8.0);
    ui.add_enabled(enabled, button).on_hover_text("Dock").clicked()
}

pub fn draw_undock(view: &View, ui: &mut Ui) -> bool {
    let button = CustomCircularImageButton::new(view, "undock", 36.0)
        .with_padding(8.0);
    ui.add(button).on_hover_text("Undock").clicked()
}