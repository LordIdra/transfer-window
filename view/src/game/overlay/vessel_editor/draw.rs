use eframe::egui::{self, style::WidgetVisuals, Color32, Context, Image, ImageButton, Pos2, Rect, Response, Rounding, Stroke, Ui};
use transfer_window_model::components::vessel_component::{system_slot::{engine::{Engine, EngineType}, fuel_tank::{FuelTank, FuelTankType}, weapon::{Weapon, WeaponType}, Slot, SlotLocation}, VesselClass};

use crate::game::Scene;

const WEAPON_SLOT_COLOR: Color32 = Color32::from_rgb(212, 11, 24);
const FUEL_TANK_SLOT_COLOR: Color32 = Color32::from_rgb(240, 200, 0);
const ENGINE_SLOT_COLOR: Color32 = Color32::from_rgb(2, 192, 240);

fn get_texture_ship_underlay(class: &VesselClass) -> &str {
    match class {
        VesselClass::Light => "ship-light",
    }
}

fn get_texture_engine(engine: Option<&Engine>) -> &str {
    match engine {
        None => "blank-slot",
        Some(engine) => match engine.type_() {
            EngineType::Efficient => "engine-efficient",
            EngineType::HighThrust => "engine-high-thrust",
        }
    }
}

fn get_texture_fuel_tank(fuel_tank: Option<&FuelTank>) -> &str {
    match fuel_tank {
        None => "blank-slot",
        Some(fuel_tank) => match fuel_tank.type_() {
            FuelTankType::Small => "tank-small",
            FuelTankType::Medium => "tank-medium",
            FuelTankType::Large => "tank-large",
        }
    }
}

fn get_texture_weapon(weapon: Option<&Weapon>) -> &str {
    match weapon {
        None => "blank-slot",
        Some(weapon) => match weapon.type_() {
            WeaponType::Torpedo => "torpedo",
        }
    }
}

fn draw_slot_from_texture(view: &mut Scene, ui: &mut Ui, texture: &str, color: Color32, location: SlotLocation, center: Pos2, size: f32, translation: f32) {
    ui.visuals_mut().widgets.inactive = WidgetVisuals {
        bg_fill: Color32::TRANSPARENT,
        weak_bg_fill: Color32::TRANSPARENT,
        bg_stroke: Stroke::new(0.08 * size, color),
        rounding: Rounding::ZERO,
        fg_stroke: Stroke::NONE,
        expansion: 0.0,
    };

    ui.visuals_mut().widgets.hovered = WidgetVisuals {
        bg_fill: Color32::from_white_alpha(10),
        weak_bg_fill: Color32::from_white_alpha(10),
        bg_stroke: Stroke::new(0.08 * size, color),
        rounding: Rounding::ZERO,
        fg_stroke: Stroke::NONE,
        expansion: 0.0,
    };

    ui.visuals_mut().widgets.active = WidgetVisuals {
        bg_fill: Color32::from_white_alpha(20),
        weak_bg_fill: Color32::from_white_alpha(20),
        bg_stroke: Stroke::new(0.08 * size, color),
        rounding: Rounding::ZERO,
        fg_stroke: Stroke::NONE,
        expansion: 0.0,
    };

    let slot_image = ImageButton::new(view.resources.get_texture_image(texture));
    let slot_position = center + egui::vec2(translation, 0.0);
    let slot_size = egui::Vec2::splat(size);
    ui.allocate_ui_at_rect(Rect::from_center_size(slot_position, slot_size), |ui| {
        if ui.add(slot_image).clicked() {
            if let Some(vessel_editor) = &mut view.vessel_editor {
                vessel_editor.slot_editor = Some(location)
            } 
        }
});
}

pub fn draw_slot(view: &mut Scene, ui: &mut Ui, slot: &Slot, location: SlotLocation, center: Pos2, size: f32, translation: f32) {
    let (texture, color) = match slot {
        Slot::Engine(engine) => (get_texture_engine(engine.as_ref()), ENGINE_SLOT_COLOR),
        Slot::FuelTank(fuel_tank) => (get_texture_fuel_tank(fuel_tank.as_ref()), FUEL_TANK_SLOT_COLOR),
        Slot::Weapon(weapon) => (get_texture_weapon(weapon.as_ref()), WEAPON_SLOT_COLOR),
    };

    draw_slot_from_texture(view, ui, texture, color, location, center, size, translation);
}

pub fn draw_ship_underlay(view: &mut Scene, context: &Context, ui: &mut Ui, class: VesselClass) -> Response {
    let texture = view.resources.get_texture_image(get_texture_ship_underlay(&class));
    let size = context.screen_rect().size() * 0.75;
    ui.add(Image::new(texture).fit_to_exact_size(size))
    
}
