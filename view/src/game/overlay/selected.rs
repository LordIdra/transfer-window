use eframe::egui::Context;
use transfer_window_model::Model;

use crate::{events::Event, game::Scene};

mod approach;
mod apsis;
mod burn;
mod encounter;
mod fire_torpedo;
mod guidance;
mod intercept;
mod orbitable;
mod point;
mod vessel;

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    approach::update(view, model, context, events);
    apsis::update(view, model, context, events);
    point::update(view, model, context, events);
    burn::update(view, model, context, events);
    encounter::update(view, model, context, events);
    guidance::update(view, model, context, events);
    orbitable::update(view, model, context);
    intercept::update(view, model, context, events);
    fire_torpedo::update(view, model, context, events);
    vessel::update(view, model, context, events);
}