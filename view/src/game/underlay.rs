use eframe::egui::Context;
use transfer_window_model::Model;

use super::Scene;

mod celestial_objects;
mod icons;
mod segments;
pub mod selected;

pub fn draw(view: &mut Scene, model: &Model, context: &Context) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw underlay");
    celestial_objects::draw(view, model);
    segments::draw(view, model);
    let is_mouse_over_any_icon = icons::draw(view, model, context);
    selected::update(view, model, context, is_mouse_over_any_icon);
}