use eframe::egui::Ui;
use transfer_window_model::Model;

pub fn draw(model: &Model, ui: &mut Ui) {
    ui.label(format!("Raw time: {}", f64::round(model.time())));
    ui.label(format!("Time step: {:?}", model.time_step()));
}