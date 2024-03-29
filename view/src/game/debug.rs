use eframe::egui::{Context, Window};
use transfer_window_model::Model;

use super::Scene;

mod overview;
mod entities;

#[derive(PartialEq)]
pub enum DebugWindowTab {
    Overview,
    Entities,
}

pub fn draw(view: &mut Scene, model: &Model, context: &Context) {
    if !view.debug_window_open {
        return;
    }
    
    Window::new("Debug")
        .show(context, |ui| {

            ui.horizontal(|ui| {
                ui.selectable_value(&mut view.debug_window_tab, DebugWindowTab::Overview, "Overview");
                ui.selectable_value(&mut view.debug_window_tab, DebugWindowTab::Entities, "Entities");
            });

            match view.debug_window_tab {
                DebugWindowTab::Overview => overview::draw(model, ui),
                DebugWindowTab::Entities => entities::draw(model, ui),
            }
    });
}