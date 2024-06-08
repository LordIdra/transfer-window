use log::trace;

use crate::game::{selected::{burn, fire_torpedo, segment_point, Selected}, View};

pub fn update(view: &mut View, is_mouse_over_any_icon: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    let is_mouse_over_ui_element = view.pointer_over_ui || is_mouse_over_any_icon;

    // IMPORTANT: the update functions may lock the context, so they
    // must not be called within an input closure, otherwise a
    // deadlock will occur!!
    let pointer = view.context.input(|input| {
        input.pointer.clone()
    });

    // Selected item deselected by clicking elsewhere
    if !is_mouse_over_ui_element && pointer.primary_clicked() {
        trace!("Selected item deselected");
        view.selected = Selected::None;
    }

    // Draw hover circle
    if !matches!(view.selected, Selected::Point { .. }) {
        segment_point::draw_hover(view, &pointer, is_mouse_over_ui_element);
    }

    match view.selected.clone() {
        Selected::None 
            | Selected::Orbitable(_) 
            | Selected::Vessel(_) 
            | Selected::Apsis { .. }
            | Selected::Approach { .. }
            | Selected::Encounter { .. }
            | Selected::Intercept { .. }
            | Selected::EnableGuidance { .. }=> (),
        Selected::Point { .. } => segment_point::draw_selected(view),
        Selected::Burn { .. } => burn::update_adjustment(view, &pointer),
        Selected::FireTorpedo { .. } => fire_torpedo::update_adjustment(view, &pointer),
    }
}