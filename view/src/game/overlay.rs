use eframe::{egui::{Color32, Context, Rounding, Stroke, Visuals}, epaint::Shadow};
use transfer_window_model::Model;

use crate::events::Event;

use super::Scene;

mod fps;
mod paused;
mod scale;
mod selected_burn;
mod selected_point;
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
    paused::update(model, context);
    scale::update(view, context);
    time::update(model, context);
    selected_point::update(view, model, context, events);
    selected_burn::update(view, model, context, events);
}
