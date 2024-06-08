use eframe::{egui::{Align2, RichText, Window}, epaint};

use crate::game::View;

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update FPS");
    Window::new("FPS")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::RIGHT_TOP, epaint::vec2(0.0, 0.0))
        .show(&view.context.clone(), |ui| {
            ui.label(RichText::new(format!("FPS: {}", view.frame_history.fps())).weak());
        });
}