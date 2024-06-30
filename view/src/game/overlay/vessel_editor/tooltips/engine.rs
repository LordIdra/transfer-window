use eframe::egui::{Grid, RichText, Ui};
use transfer_window_model::components::vessel_component::system_slot::{engine::{Engine, EngineType}, System};

use crate::game::View;

pub fn show_tooltip(view: &View, ui: &mut Ui, engine: &Option<Engine>) {
    let Some(engine) = engine else {
        ui.label(RichText::new("None").strong().monospace().size(20.0));
        return;
    };

    let type_ = engine.type_();
    let name = match type_ {
        EngineType::Torpedo => unreachable!(),
        EngineType::Efficient => "Efficient Engine",
        EngineType::HighThrust => "High Thrust Engine",
    };


    ui.label(RichText::new(name).strong().monospace().size(20.0));

    Grid::new("Tooltip for ".to_string() + name).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.image(view.resources.icon_image("fuel"));
            ui.label(RichText::new("Fuel Consumption").monospace().strong());
        });
        ui.label(format!("{} L/s", type_.fuel_kg_per_second()));
        ui.end_row();

        ui.horizontal(|ui| {
            ui.image(view.resources.icon_image("thrust"));
            ui.label(RichText::new("Thrust").monospace().strong());
        });
        ui.label(format!("{} N", type_.thrust_newtons()));
        ui.end_row();

        ui.horizontal(|ui| {
            ui.image(view.resources.icon_image("isp"));
            ui.label(RichText::new("Specific Impulse").monospace().strong());
        });
        ui.label(format!("{} s", type_.specific_impulse_space().round()));
        ui.end_row();
    });
}