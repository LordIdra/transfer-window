use eframe::egui::{self, Color32, Grid, Image, ImageButton, Pos2, Rect, Response, RichText, Ui};
use transfer_window_model::{components::{path_component::burn::rocket_equation_function::RocketEquationFunction, vessel_component::ship::{ship_slot::{ShipSlot, ShipSlotLocation}, Ship, ShipClass}}, storage::entity_allocator::Entity};

use crate::{game::{events::ViewEvent, overlay::slot_textures::TexturedSlot, View}, styles};

use super::util::{compute_slot_locations, compute_slot_size};

const UNDERLAY_SIZE_PROPORTION: f32 = 0.7;
const WEAPON_SLOT_COLOR: Color32 = Color32::from_rgb(212, 11, 24);
const FUEL_TANK_SLOT_COLOR: Color32 = Color32::from_rgb(240, 200, 0);
const ENGINE_SLOT_COLOR: Color32 = Color32::from_rgb(2, 192, 240);

fn compute_texture_ship_underlay(class: &ShipClass) -> &str {
    match class {
        ShipClass::Scout => "ship-scout",
        ShipClass::Frigate => "ship-frigate",
    }
}

fn on_slot_clicked(view: &View, location: ShipSlotLocation) {
    if let Some(vessel_editor) = &view.vessel_editor {
        let slot_editor = if vessel_editor.slot_editor == Some(location) {
            None
        } else {
            Some(location)
        };

        let mut vessel_editor = view.vessel_editor.as_ref().unwrap().clone();
        vessel_editor.slot_editor = slot_editor;
        view.add_view_event(ViewEvent::SetVesselEditor(Some(vessel_editor)));
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_slot_from_texture(view: &View, ui: &mut Ui, texture: &str, color: Color32, location: ShipSlotLocation, center: Pos2, size: f32, translation: f32) {
    let slot_position = center + egui::vec2(translation, 0.0);
    let slot_size = egui::Vec2::splat(size);
    let slot_rect = Rect::from_center_size(slot_position, slot_size);
    
    styles::SlotEditor::apply(ui, size, color);
    ui.allocate_ui_at_rect(slot_rect, |ui| {
        let slot_image = ImageButton::new(view.resources.texture_image(texture));
        if ui.add(slot_image).clicked() {
            on_slot_clicked(view, location);
        }
});
}

fn draw_slot(view: &View, ui: &mut Ui, slot: &ShipSlot, location: ShipSlotLocation, center: Pos2, size: f32, translation: f32) {
    let texture = slot.texture();
    let color = match slot {
        ShipSlot::Engine(_) => ENGINE_SLOT_COLOR,
        ShipSlot::FuelTank(_) => FUEL_TANK_SLOT_COLOR,
        ShipSlot::Weapon(_) => WEAPON_SLOT_COLOR,
    };

    draw_slot_from_texture(view, ui, texture, color, location, center, size, translation);
}

fn draw_ship_underlay(view: &View, ui: &mut Ui, class: ShipClass) -> Response {
    let texture = view.resources.texture_image(compute_texture_ship_underlay(&class));
    let size = view.screen_rect.size() * UNDERLAY_SIZE_PROPORTION;
    ui.add(Image::new(texture).fit_to_exact_size(size))
}

fn draw_ship_stats(ui: &mut Ui, ship: &Ship, vessel_name: &str) {
    ui.vertical(|ui| {
        ui.set_width(200.0);
    
        Grid::new("Ship stats - ".to_string() + vessel_name).show(ui, |ui| {
            ui.label(RichText::new("Class").monospace().strong());
            ui.label(ship.class().name());
            ui.end_row();
        
            ui.label(RichText::new("Dry mass").monospace().strong());
            ui.label(format!("{} kg", ship.dry_mass()));
            ui.end_row();
            
            if !ship.fuel_tanks().is_empty() {
                ui.label(RichText::new("Wet mass").monospace().strong());
                ui.label(format!("{} kg", ship.wet_mass().round()));
                ui.end_row();

                ui.label(RichText::new("Fuel capacity").monospace().strong());
                ui.label(format!("{} L", ship.max_fuel_litres()));
                ui.end_row();
            

                if ship.engine().is_some() {
                    let rocket_equation_function = RocketEquationFunction::new(ship.dry_mass(), ship.max_fuel_kg(), ship.fuel_kg_per_second().unwrap(), ship.specific_impulse().unwrap(), 0.0);

                    ui.label(RichText::new("Max ΔV").monospace().strong());
                    ui.label(format!("{} m/s", rocket_equation_function.remaining_dv().round()));
                    ui.end_row();
            
                    ui.label(RichText::new("Acceleration (wet)").monospace().strong());
                    ui.label(format!("{:.2} m/s", rocket_equation_function.acceleration()));
                    ui.end_row();

                    ui.label(RichText::new("Acceleration (dry)").monospace().strong());
                    ui.label(format!("{:.2} m/s", rocket_equation_function.end().acceleration()));
                    ui.end_row();
                }
            }

        });
    });
}

pub fn draw_vessel_editor(view: &View, ui: &mut Ui, vessel_name: &str, entity: Entity) -> Rect {
    ui.horizontal(|ui| {
        let vessel_component = view.model.vessel_component(entity);
        let ship = vessel_component.as_ship().expect("Attempt to open vessel editor for non-ship vessel");
        draw_ship_stats(ui, ship, vessel_name);
        let response = draw_ship_underlay(view, ui, ship.class());
        let center = response.rect.center();
        let size = response.rect.size();
        let slot_size = compute_slot_size(ship.class()) * size.x;
        for (slot_location, translation) in compute_slot_locations(ship.class()) {
            let slot = ship.get_slot(slot_location).clone();
            draw_slot(view, ui, &slot, slot_location, center, slot_size, translation * size.x);
        }
        response.rect
    }).inner
}