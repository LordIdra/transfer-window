use eframe::egui::Context;
use transfer_window_model::Model;

use crate::{events::Event, game::Scene};

mod burn;
mod fire_torpedo;
mod guidance;
mod orbitable;
mod point;
mod vessel;

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    point::update(view, model, context, events);
    burn::update(view, model, context, events);
    guidance::update(view, model, context, events);
    orbitable::update(view, model, context);
    fire_torpedo::update(view, model, context, events);
    vessel::update(view, model, context, events);
}