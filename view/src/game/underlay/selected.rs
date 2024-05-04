use eframe::egui::Context;

use log::trace;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::Scene};

use self::burn::BurnState;

pub mod burn;
pub mod segment_point;

#[derive(Debug, Clone)]
pub enum Selected {
    None,
    Orbitable(Entity),
    Vessel(Entity),
    Point { entity: Entity, time: f64 },
    Burn { entity: Entity, time: f64, state: BurnState }
}

impl Selected {
    pub fn selected_entity(&self) -> Option<Entity> {
        match self {
            Selected::None => None,
            Selected::Orbitable(entity) 
                | Selected::Vessel(entity) 
                | Selected::Burn { entity, time: _, state: _ }
                | Selected::Point { entity, time: _ }
                => Some(*entity),
        }
    }
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, is_mouse_over_any_icon: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    let is_mouse_over_ui_element = context.is_pointer_over_area() || is_mouse_over_any_icon;

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
        Selected::Point { entity: _, time: _ } => segment_point::draw_selected(view, model),
        Selected::Burn { entity: _, time: _, state: _ } => burn::update_drag(view, model, context, events, &pointer),
        _ => ()
    }
}