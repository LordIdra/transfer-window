//! The overlay constitutes everything that *does not* move with the camera

use eframe::egui::{Context, Style};
use transfer_window_model::Model;

use crate::events::Event;

use super::Scene;

mod fps;
mod scale;
mod selected;
mod time;
pub mod vessel;
pub mod widgets;

pub fn draw(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw overlay");

    view.styles.default_window_visuals.apply(context);
    
    fps::update(view, context);
    scale::update(view, context);
    time::update(model, context);
    selected::update(view, model, context, events);

    view.styles.vessel_editor_visuals.apply(context);

    vessel::update(view, model, context, events);

    context.set_style(Style::default());
}
