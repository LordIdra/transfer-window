use eframe::egui::Ui;
use transfer_window_model::components::vessel_component::system_slot::{weapon::{Weapon, WeaponType}, System};

pub fn show_tooltip(ui: &mut Ui, weapon: &Option<Weapon>) {
    let Some(weapon) = weapon else {
        ui.label("None");
        return;
    };

    let name = match weapon.type_() {
        WeaponType::Torpedo => "Torpedo",
    };

    ui.label(name);
}