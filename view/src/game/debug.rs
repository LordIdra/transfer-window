use eframe::egui::Window;

use super::View;

mod overview;
mod entities;

#[derive(PartialEq)]
pub enum DebugWindowTab {
    Overview,
    Entities,
}

pub fn draw(view: &mut View) {
    if !view.debug_window_open {
        return;
    }
    
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw debug");
    
    Window::new("Debug")
        .show(&view.context.clone(), |ui| {

            ui.horizontal(|ui| {
                ui.selectable_value(&mut view.debug_window_tab, DebugWindowTab::Overview, "Overview");
                ui.selectable_value(&mut view.debug_window_tab, DebugWindowTab::Entities, "Entities");
            });

            match view.debug_window_tab {
                DebugWindowTab::Overview => overview::draw(&view.model, ui),
                DebugWindowTab::Entities => entities::draw(&view.model, ui),
            }
    });
}