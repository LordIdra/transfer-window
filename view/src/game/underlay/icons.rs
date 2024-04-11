use eframe::egui::Context;
use transfer_window_model::Model;

use crate::game::Scene;

mod object_icons;

/// Returns true if any icon is hovered over
pub fn draw(view: &mut Scene, model: &Model, context: &Context) -> bool {
    object_icons::draw(view, model, context)
}