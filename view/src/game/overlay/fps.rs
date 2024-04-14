use eframe::{egui::{Align2, Context, Window}, epaint};

use crate::game::Scene;

pub fn update(view: &Scene, context: &Context) {
    Window::new("FPS")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::RIGHT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label("FPS: ".to_string() + view.frame_history.fps().to_string().as_str());
        });
}