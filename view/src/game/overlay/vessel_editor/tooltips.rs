use eframe::egui::Ui;
use transfer_window_model::components::vessel_component::system_slot::Slot;

mod engine;
mod fuel_tank;
mod weapon;

pub fn show_tooltip(ui: &mut Ui, slot: &Slot) {
    match slot {
        Slot::Weapon(weapon) => weapon::show_tooltip(ui, weapon),
        Slot::FuelTank(fuel_tank) => fuel_tank::show_tooltip(ui, fuel_tank),
        Slot::Engine(engine) => engine::show_tooltip(ui, engine),
    };
}