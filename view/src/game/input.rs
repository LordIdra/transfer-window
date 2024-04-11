use eframe::egui::Context;

use crate::events::Event;

use super::Scene;

mod keyboard;
mod mouse;

pub fn update(view: &mut Scene, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update input");
    keyboard::update(view, context, events);
    mouse::update(view, context);
}