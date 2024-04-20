use eframe::{egui::{self, style::WidgetVisuals, Align2, Color32, Context, Image, ImageButton, ImageSource, Pos2, Rect, Response, Rounding, Stroke, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::{system_slot::{engine::{Engine, EngineType}, fuel_tank::{FuelTank, FuelTankType}, weapon::{Weapon, WeaponType}, LightSlots, SystemSlots}, VesselClass}, Model};

use crate::game::Scene;

const LIGHT_SLOT_SIZE: f32 = 0.113;
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

fn render_slot(ui: &mut Ui, texture: ImageSource, color: Color32, center: Pos2, size: f32, translation: f32) {
    ui.visuals_mut().widgets.inactive = WidgetVisuals {
        bg_fill: Color32::TRANSPARENT,
        weak_bg_fill: Color32::TRANSPARENT,
        bg_stroke: Stroke::new(5.0, color),
        rounding: Rounding::ZERO,
        fg_stroke: Stroke::NONE,
        expansion: 0.0,
    };

    ui.visuals_mut().widgets.hovered = WidgetVisuals {
        bg_fill: Color32::from_white_alpha(10),
        weak_bg_fill: Color32::from_white_alpha(10),
        bg_stroke: Stroke::new(5.0, color),
        rounding: Rounding::ZERO,
        fg_stroke: Stroke::NONE,
        expansion: 0.0,
    };

    ui.visuals_mut().widgets.active = WidgetVisuals {
        bg_fill: Color32::from_white_alpha(20),
        weak_bg_fill: Color32::from_white_alpha(20),
        bg_stroke: Stroke::new(5.0, color),
        rounding: Rounding::ZERO,
        fg_stroke: Stroke::NONE,
        expansion: 0.0,
    };

    let slot_image = ImageButton::new(texture);
    let slot_position = center + egui::vec2(translation, 0.0);
    let slot_size = egui::Vec2::splat(size);
    ui.allocate_ui_at_rect(Rect::from_center_size(slot_position, slot_size), |ui| ui.add(slot_image));
}

fn render_engine(view: &Scene, ui: &mut Ui, engine: Option<&Engine>, center: Pos2, size: f32, translation: f32) {
    let texture = view.resources.get_texture_image(get_texture_engine(engine));
    render_slot(ui, texture, ENGINE_SLOT_COLOR, center, translation, size)
}

fn render_fuel_tank(view: &Scene, ui: &mut Ui, fuel_tank: Option<&FuelTank>, center: Pos2, size: f32, translation: f32) {
    let texture = view.resources.get_texture_image(get_texture_fuel_tank(fuel_tank));
    render_slot(ui, texture, FUEL_TANK_SLOT_COLOR, center, translation, size)
}

fn render_weapon(view: &Scene, ui: &mut Ui, weapon: Option<&Weapon>, center: Pos2, size: f32, translation: f32) {
    let texture = view.resources.get_texture_image(get_texture_weapon(weapon));
    render_slot(ui, texture, WEAPON_SLOT_COLOR, center, translation, size)
}

fn render_ship_underlay(view: &Scene, context: &Context, ui: &mut Ui, class: VesselClass) -> Response {
    let texture = view.resources.get_texture_image(get_texture_ship_underlay(&class));
    let size = context.screen_rect().size() * 0.75;
    ui.add(Image::new(texture).fit_to_exact_size(size))
    
}

fn render_light(view: &Scene, context: &Context, ui: &mut Ui, slots: &LightSlots) {
    let response = render_ship_underlay(view, context, ui, VesselClass::Light);
    let center = response.rect.center();
    let size = response.rect.size();

    render_weapon(view, ui, slots.weapon(), center, -0.142 * size.x, LIGHT_SLOT_SIZE * size.x);
    render_fuel_tank(view, ui, slots.fuel_tank(), center, 0.177 * size.x, LIGHT_SLOT_SIZE * size.x);
    render_engine(view, ui, slots.engine(), center, 0.382 * size.x, LIGHT_SLOT_SIZE * size.x);
}

pub fn update(view: &Scene, model: &Model, context: &Context) {
    let Some(entity) = view.vessel_editor else {
        return;
    };

    Window::new("Vessel editor")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label(model.get_name_component(entity).get_name().to_uppercase());
            let vessel_component = model.get_vessel_component(entity);
            match vessel_component.get_slots() {
                SystemSlots::Light(slots) => render_light(view, context, ui, slots),
            }
        });
}