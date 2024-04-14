use eframe::egui::Context;
use transfer_window_model::Model;

use crate::events::Event;

use super::Scene;

mod fps;
mod paused;
mod time;
mod selected_point;
mod selected_burn;

pub fn draw(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw overlay");
    
    fps::update(view, context);
    paused::update(model, context);
    time::update(context, model);
    selected_point::update(view, context, model, events);
    selected_burn::update(view, context, model, events);
}
