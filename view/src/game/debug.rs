use eframe::egui::Window;

use super::events::ViewEvent;
use super::View;

mod entities;
mod overview;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebugWindowTab {
    Model,
    Entities,
    Gui,
}

pub fn draw(view: &View) {
    if !view.debug_window_open {
        return;
    }

    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw debug");

    Window::new("Debug").show(&view.context.clone(), |ui| {
        let mut debug_window_tab = view.debug_window_tab;
        ui.horizontal(|ui| {
            ui.selectable_value(&mut debug_window_tab, DebugWindowTab::Model, "Model");
            ui.selectable_value(&mut debug_window_tab, DebugWindowTab::Entities, "Entities");
            ui.selectable_value(&mut debug_window_tab, DebugWindowTab::Gui, "GUI");
        });
        view.add_view_event(ViewEvent::SetDebugWindowTab(debug_window_tab));

        match view.debug_window_tab {
            DebugWindowTab::Model => overview::draw(view, ui),
            DebugWindowTab::Entities => entities::draw(&view.model, ui),
            DebugWindowTab::Gui => view.context.inspection_ui(ui),
        }
    });
}
