use super::super::util::format_time;

use eframe::{egui::{Align2, Context, Window}, epaint};

use transfer_window_model::Model;

pub fn update(model: &Model, context: &Context) {
    Window::new("Time")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::CENTER_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label("Time: ".to_string() + format_time(model.get_time()).as_str());
            ui.label("Time step: ".to_string() + format_time(model.get_time_step().get_time_step()).as_str());
            if model.get_time_step().is_paused() {
                ui.label("PAUSED");
            }
        });
}
