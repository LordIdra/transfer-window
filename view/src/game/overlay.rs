use eframe::{egui::{Align2, Context, Window}, epaint};
use transfer_window_model::Model;

use super::util::format_time;

pub fn draw(model: &Model, context: &Context) {
    if model.get_time_step().is_paused() {
        Window::new("Paused")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::CENTER_BOTTOM, epaint::vec2(0.0, -30.0))
            .show(context, |ui| {
                ui.label("SIMULATION PAUSED")
        });
    }

    Window::new("Time")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::CENTER_TOP, epaint::vec2(0.0, 30.0))
        .show(context, |ui| {
            ui.label("Time: ".to_string() + format_time(model.get_time()).as_str())
    });
}