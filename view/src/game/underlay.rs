use eframe::egui::Context;
use transfer_window_model::Model;

use crate::events::Event;

use super::Scene;

mod celestial_objects;
mod icons;
mod segments;
mod selected;

pub fn draw(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) -> bool {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw underlay");
    celestial_objects::draw(view, model);
    segments::draw(view, model);
    let is_mouse_over_any_icon = icons::draw(view, model, context);
    selected::update(view, model, context, events, is_mouse_over_any_icon);
    is_mouse_over_any_icon
}