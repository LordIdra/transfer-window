use eframe::egui::Context;

use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::Scene};

use self::{burn::BurnState, segment_point::SegmentPointState};

pub mod burn;
pub mod segment_point;

#[derive(Debug, Clone)]
pub enum Selected {
    None,
    Point { entity: Entity, time: f64, state: SegmentPointState },
    Burn { entity: Entity, time: f64, state: BurnState }
}

pub fn remove_if_expired(view: &mut Scene, model: &Model) {
    match view.selected.clone() {
        Selected::None => (),
        Selected::Point { entity: _, time, state } => segment_point::remove_if_expired(view, model, time, &state),
        Selected::Burn { entity: _, time, state: _ } => burn::remove_if_expired(view, model, time),
    }
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, is_mouse_over_any_icon: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    let is_mouse_over_ui_element = context.is_pointer_over_area() || is_mouse_over_any_icon;
    context.input(|input| {
        match view.selected.clone() {
            Selected::None => segment_point::update_not_selected(view, model, context, &input.pointer, is_mouse_over_ui_element),
            Selected::Point { entity, time, state } => segment_point::update_selected(view, model, context, &input.pointer, is_mouse_over_ui_element, entity, time, &state),
            Selected::Burn { entity: _, time: _, state: _ } => burn::update_selected(view, model, events, &input.pointer, is_mouse_over_ui_element, context.screen_rect()),
        }
    });

    segment_point::draw(view, model);
}