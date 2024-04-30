use eframe::egui::Context;
use transfer_window_model::Model;

use crate::{events::Event, game::Scene};

mod burn;
mod point;
mod vessel;

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    point::update(view, model, context, events);
    burn::update(view, model, context, events);
    vessel::update(view, model, context, events);
}