use eframe::{egui::{Align2, Context, Window}, epaint};
use transfer_window_model::Model;

pub fn update(model: &Model, context: &Context) {
    if !model.get_time_step().is_paused() {
        return;
    }
    
    Window::new("Paused")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::CENTER_BOTTOM, epaint::vec2(0.0, -30.0))
        .show(context, |ui| {
            ui.label("SIMULATION PAUSED")
        });
}