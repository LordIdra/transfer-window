use eframe::egui::{RichText, Ui};
use transfer_window_model::components::vessel_component::ship::ship_slot::{weapon::{Weapon, WeaponType}, System};

pub fn show_tooltip(ui: &mut Ui, weapon: &Option<Weapon>) {
    let Some(weapon) = weapon else {
        ui.label(RichText::new("None").strong().monospace().size(20.0));
        return;
    };

    let name = match weapon.type_() {
        WeaponType::Torpedo(_) => "Torpedo",
    };

    ui.label(RichText::new(name).strong().monospace().size(20.0));
}