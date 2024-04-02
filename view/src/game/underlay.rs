use eframe::egui::Context;
use transfer_window_model::Model;

use super::Scene;

mod celestial_objects;
mod icons;
mod segments;

pub fn draw(view: &mut Scene, model: &Model, context: &Context) {
    celestial_objects::draw(view, model);
    icons::draw(view, model, context);
    segments::draw(view, model, context);
}