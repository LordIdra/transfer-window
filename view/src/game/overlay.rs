use eframe::egui::Context;
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
    
    fps::update(view, context);
    paused::update(model, context);
    scale::update(view, context);
    time::update(model, context);
    selected_point::update(view, model, context, events);
    selected_burn::update(view, model, context, events);
}
