use log::trace;
use transfer_window_model::components::vessel_component::faction::Faction;

use crate::game::{events::ViewEvent, selected::{burn, fire_torpedo, segment_point::{self, SELECT_DISTANCE}, Selected}, View};

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");

    // IMPORTANT: the update functions may lock the context, so they
    // must not be called within an input closure, otherwise a
    // deadlock will occur!!
    let pointer = view.context.input(|input| {
        input.pointer.clone()
    });

    // Selected item deselected by clicking elsewhere
    if !(view.pointer_over_ui || view.pointer_over_icon) && pointer.primary_clicked() {
        trace!("Selected item deselected");
        view.add_view_event(ViewEvent::SetSelected(Selected::None));
    }

    // Draw hover circle
    if !matches!(view.selected, Selected::OrbitPoint { .. }) {
        segment_point::draw_hover(view, &pointer);
    }

    // Select point
    if pointer.primary_clicked() && !view.pointer_over_ui && !view.pointer_over_icon {
        let select_distance = SELECT_DISTANCE / view.camera.zoom();
        if let Some(latest_window) = pointer.latest_pos() { 
            let latest_world = view.window_space_to_world_space(latest_window);

            if let Some((entity, time)) = view.model.closest_burn_point(latest_world, select_distance, Some(Faction::Player)) {
                trace!("Selected orbit point at time={}", time);
                let selected = Selected::BurnPoint { entity, time };
                view.add_view_event(ViewEvent::SetSelected(selected));
            } else if let Some((entity, time)) = view.model.closest_guidance_point(latest_world, select_distance, Some(Faction::Player)) {
                trace!("Selected orbit point at time={}", time);
                let selected = Selected::GuidancePoint { entity, time };
                view.add_view_event(ViewEvent::SetSelected(selected));
            } else if let Some((entity, time)) = view.model.closest_orbit_point(latest_world, select_distance, Some(Faction::Player)) {
                trace!("Selected orbit point at time={}", time);
                let selected = Selected::OrbitPoint { entity, time };
                view.add_view_event(ViewEvent::SetSelected(selected));
            }
        }
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
        Selected::BurnPoint { .. }
            | Selected::GuidancePoint { .. } 
            | Selected::OrbitPoint { .. } => segment_point::draw_selected(view),
        Selected::Burn { .. } => burn::update_adjustment(view, &pointer),
        Selected::FireTorpedo { .. } => fire_torpedo::update_adjustment(view, &pointer),
    }
}