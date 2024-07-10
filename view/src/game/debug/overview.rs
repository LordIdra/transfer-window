use eframe::egui::Ui;

use crate::game::events::ModelEvent;
use crate::game::View;

pub fn draw(view: &View, ui: &mut Ui) {
    ui.label(format!("Raw time: {}", f64::round(view.model.time())));
    ui.label(format!("Time step: {:?}", view.model.time_step()));
    if let Some(warp) = view.model.warp() {
        ui.label(format!("Warp: {warp:?}"));
    }
    if ui.button("Save").clicked() {
        view.add_model_event(ModelEvent::SaveGame {
            name: "debug".to_string(),
        });
    }
}
