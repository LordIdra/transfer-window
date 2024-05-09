use eframe::egui::Context;
use transfer_window_model::Model;

use crate::events::Event;

use super::Scene;

mod camera;
mod keyboard;

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update misc");
    camera::update(view, model, context);
    keyboard::update(view, context, events);
}