use eframe::egui::Context;
use transfer_window_model::Model;

use crate::events::Event;

use super::Scene;

mod celestial_objects;
mod icons;

pub fn draw(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    celestial_objects::draw(view, model);
    icons::draw(view, model, context, events);
}