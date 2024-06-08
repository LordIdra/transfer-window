use eframe::egui::Ui;
use transfer_window_model::components::vessel_component::system_slot::Slot;

use crate::game::View;

mod engine;
mod fuel_tank;
mod weapon;

pub fn show_tooltip(view: &View, ui: &mut Ui, slot: &Slot) {
    match slot {
        Slot::Weapon(weapon) => weapon::show_tooltip(ui, weapon),
        Slot::FuelTank(fuel_tank) => fuel_tank::show_tooltip(view, ui, fuel_tank),
        Slot::Engine(engine) => engine::show_tooltip(view, ui, engine),
    };
}