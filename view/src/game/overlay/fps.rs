use eframe::{egui::{Align2, Context, RichText, Window}, epaint};

use crate::game::Scene;

pub fn update(view: &Scene, context: &Context) {
    Window::new("FPS")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::RIGHT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label(RichText::new(format!("FPS: {}", view.frame_history.fps())).weak());
        });
}