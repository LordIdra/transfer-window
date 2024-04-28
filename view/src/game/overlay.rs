//! The overlay constitutes everything that *does not* move with the camera

use eframe::{egui::{style::Spacing, Color32, Context, Margin, Rounding, Stroke, Style, Visuals}, epaint::Shadow};
use transfer_window_model::Model;

use crate::events::Event;

use super::Scene;

mod fps;
mod scale;
mod selected;
mod time;
pub mod vessel;

pub fn draw(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw overlay");

    context.set_visuals(Visuals {
        window_fill: Color32::from_rgba_unmultiplied(0, 0, 0, 100),
        window_stroke: Stroke::NONE,
        window_shadow: Shadow::NONE,
        window_rounding: Rounding::ZERO,
        ..Visuals::default()
    });
    
    fps::update(view, context);
    scale::update(view, context);
    time::update(model, context);
    selected::update(view, model, context, events);

    context.set_visuals(Visuals {
        window_fill: Color32::from_rgba_unmultiplied(0, 0, 0, 200),
        window_stroke: Stroke::NONE,
        window_shadow: Shadow::NONE,
        window_rounding: Rounding::ZERO,
        ..Visuals::default()
    });

    context.set_style(Style {
        spacing: Spacing {
            window_margin: Margin::same(20.0),
            ..Spacing::default()
        },
        ..Style::default()
    });

    vessel::update(view, model, context, events);

    context.set_style(Style::default());
}
