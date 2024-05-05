use eframe::egui::{RichText, Ui};
use transfer_window_model::components::vessel_component::system_slot::{fuel_tank::{FuelTank, FuelTankType}, System};

use crate::icons::ICON_OIL_BARREL;

pub fn show_tooltip(ui: &mut Ui, fuel_tank: &Option<FuelTank>) {
    let Some(fuel_tank) = fuel_tank else {
        ui.label("None");
        return;
    };

    let type_ = fuel_tank.type_();
    let name = match type_ {
        FuelTankType::Torpedo => unreachable!(),
        FuelTankType::Small => "Small Fuel Tank",
        FuelTankType::Medium => "Medium Fuel Tank",
        FuelTankType::Large => "Large Fuel Tank",
    };

    ui.label(name);
    ui.label(RichText::new(format!("{} Capacity: {} L", ICON_OIL_BARREL, type_.capacity_litres())));
}