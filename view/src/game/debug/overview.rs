use eframe::egui::Ui;

use crate::game::{events::Event, View};

pub fn draw(view: &mut View, ui: &mut Ui) {
    ui.label(format!("Raw time: {}", f64::round(view.model.time())));
    ui.label(format!("Time step: {:?}", view.model.time_step()));
    if let Some(warp) = view.model.warp() {
        ui.label(format!("Warp: {:?}", warp));
    }
    if ui.button("Save").clicked() {
        view.events.push(Event::SaveGame { name: "debug".to_string() })
    }
}