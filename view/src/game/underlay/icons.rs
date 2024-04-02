use eframe::egui::Context;
use transfer_window_model::Model;

use crate::game::Scene;

mod object_icons;

pub fn draw(view: &mut Scene, model: &Model, context: &Context) {
    object_icons::draw(view, model, context);
}