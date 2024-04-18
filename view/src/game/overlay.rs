//! The overlay constitutes everything that *does not* move with the camera

use eframe::{egui::{Color32, Context, Rounding, Stroke, Visuals}, epaint::Shadow};
use transfer_window_model::Model;

use crate::events::Event;

use super::Scene;

mod fps;
mod scale;
mod selected;
mod time;

pub fn draw(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw overlay");

    let default = context.style().visuals.clone();
    context.set_visuals(Visuals {
        window_fill: Color32::from_rgba_unmultiplied(0, 0, 0, 100),
        window_stroke: Stroke::NONE,
        window_shadow: Shadow::NONE,
        window_rounding: Rounding::ZERO,
        ..default
    });
    
    fps::update(view, context);
    scale::update(view, context);
    time::update(model, context);
    selected::update(view, model, context, events);
}
