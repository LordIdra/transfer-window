use eframe::egui::Context;
use log::trace;
use transfer_window_model::Model;

use crate::{events::Event, game::{selected::{burn, fire_torpedo, segment_point, Selected}, Scene}};

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, is_mouse_over_any_icon: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    let is_mouse_over_ui_element = view.pointer_over_ui_last_frame || is_mouse_over_any_icon;

    // IMPORTANT: the update functions may lock the context, so they
    // must not be called within an input closure, otherwise a
    // deadlock will occur!!
    let pointer = context.input(|input| {
        input.pointer.clone()
    });

    // Selected item deselected by clicking elsewhere
    if !is_mouse_over_ui_element && pointer.primary_clicked() {
        trace!("Selected item deselected");
        view.selected = Selected::None;
    }

    // Draw hover circle
    if !matches!(view.selected, Selected::Point { entity: _, time: _ }) {
        segment_point::draw_hover(view, model, context, &pointer, is_mouse_over_ui_element);
    }

    match view.selected.clone() {
        Selected::None 
            | Selected::Orbitable(_) 
            | Selected::Vessel(_) 
            | Selected::EnableGuidance { entity: _, time: _ }=> (),
        Selected::Point { entity: _, time: _ } => segment_point::draw_selected(view, model),
        Selected::Burn { entity: _, time: _, state: _ } => burn::update_adjustment(view, model, context, events, &pointer),
        Selected::FireTorpedo { entity: _, time: _, state: _ } => fire_torpedo::update_adjustment(view, model, context, events, &pointer),
    }
}