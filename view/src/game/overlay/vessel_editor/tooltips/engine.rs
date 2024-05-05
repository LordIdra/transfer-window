use eframe::egui::{RichText, Ui};
use transfer_window_model::components::vessel_component::system_slot::{engine::{Engine, EngineType}, System};

use crate::icons::{ICON_FAST_FORWARD, ICON_OIL_BARREL, ICON_PIE_CHART_OUTLINE};

pub fn show_tooltip(ui: &mut Ui, engine: &Option<Engine>) {
    let Some(engine) = engine else {
        ui.label("None");
        return;
    };

    let type_ = engine.type_();
    let name = match type_ {
        EngineType::Torpedo => unreachable!(),
        EngineType::Efficient => "Efficient Engine",
        EngineType::HighThrust => "High Thrust Engine",
    };


    ui.label(name);
    ui.label(RichText::new(format!("{} Fuel Consumption: {} L/s", ICON_OIL_BARREL, type_.fuel_kg_per_second())));
    ui.label(RichText::new(format!("{} Thrust: {} kN", ICON_FAST_FORWARD, type_.thrust_newtons() / 1000.0)));
    ui.label(RichText::new(format!("{} Specific Impulse (vacuum): {} s", ICON_PIE_CHART_OUTLINE, type_.specific_impulse_space().round())));
}