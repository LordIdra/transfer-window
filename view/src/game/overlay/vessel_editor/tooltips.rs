use eframe::egui::Ui;
use transfer_window_model::components::vessel_component::ship::ship_slot::ShipSlot;
use crate::game::View;

mod engine;
mod fuel_tank;
mod weapon;

pub fn show_tooltip(view: &View, ui: &mut Ui, slot: &ShipSlot) {
    match slot {
        ShipSlot::Weapon(weapon) => weapon::show_tooltip(ui, weapon),
        ShipSlot::FuelTank(fuel_tank) => fuel_tank::show_tooltip(view, ui, fuel_tank),
        ShipSlot::Engine(engine) => engine::show_tooltip(view, ui, engine),
    };
}