use eframe::egui::{RichText, Ui};
use transfer_window_model::components::vessel_component::system_slot::{fuel_tank::{FuelTank, FuelTankType}, System};

use crate::game::View;


pub fn show_tooltip(view: &View, ui: &mut Ui, fuel_tank: &Option<FuelTank>) {
    let Some(fuel_tank) = fuel_tank else {
        ui.label(RichText::new("None").strong().monospace().size(20.0));
        return;
    };

    let type_ = fuel_tank.type_();
    let name = match type_ {
        FuelTankType::Torpedo => unreachable!(),
        FuelTankType::Small => "Small Fuel Tank",
        FuelTankType::Medium => "Medium Fuel Tank",
        FuelTankType::Large => "Large Fuel Tank",
    };

    ui.label(RichText::new(name).strong().monospace().size(20.0));
    ui.horizontal(|ui| {
        ui.image(view.resources.icon_image("fuel"));
        ui.label(RichText::new("Capacity").monospace().strong());
        ui.label(format!("{} L", type_.capacity_litres()));
    });
}