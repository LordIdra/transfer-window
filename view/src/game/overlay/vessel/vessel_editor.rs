use eframe::egui::{self, style::WidgetVisuals, Color32, Context, Image, ImageButton, Pos2, Rect, Response, Rounding, Stroke, Ui};
use transfer_window_model::components::vessel_component::{system_slot::{Slot, SlotLocation, Slots}, VesselClass};

use crate::game::Scene;

use super::util::{get_slot_locations, get_slot_size, TexturedSlot};

const UNDERLAY_SIZE_PROPORTION: f32 = 0.75;
const WEAPON_SLOT_COLOR: Color32 = Color32::from_rgb(212, 11, 24);
const FUEL_TANK_SLOT_COLOR: Color32 = Color32::from_rgb(240, 200, 0);
const ENGINE_SLOT_COLOR: Color32 = Color32::from_rgb(2, 192, 240);

fn get_texture_ship_underlay(class: &VesselClass) -> &str {
    match class {
        VesselClass::Light => "ship-light",
    }
}

#[allow(clippy::too_many_arguments)]
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
                if vessel_editor.slot_editor == Some(location) {
                    vessel_editor.slot_editor = None;
                } else {
                    vessel_editor.slot_editor = Some(location);
                }
            } 
        }
});
}

fn draw_slot(view: &mut Scene, ui: &mut Ui, slot: &Slot, location: SlotLocation, center: Pos2, size: f32, translation: f32) {
    let texture = slot.get_texture();
    let color = match slot {
        Slot::Engine(_) => ENGINE_SLOT_COLOR,
        Slot::FuelTank(_) => FUEL_TANK_SLOT_COLOR,
        Slot::Weapon(_) => WEAPON_SLOT_COLOR,
    };

    draw_slot_from_texture(view, ui, texture, color, location, center, size, translation);
}

fn draw_ship_underlay(view: &mut Scene, context: &Context, ui: &mut Ui, class: VesselClass) -> Response {
    let texture = view.resources.get_texture_image(get_texture_ship_underlay(&class));
    let size = context.screen_rect().size() * UNDERLAY_SIZE_PROPORTION;
    ui.add(Image::new(texture).fit_to_exact_size(size))
}

pub fn draw_vessel_editor(view: &mut Scene, context: &Context, ui: &mut Ui, vessel_class: VesselClass, slots: &Slots) -> Rect {
    let response = draw_ship_underlay(view, context, ui, VesselClass::Light);
    let center = response.rect.center();
    let size = response.rect.size();
    let slot_size = get_slot_size(vessel_class) * size.x;
    for (slot_location, translation) in get_slot_locations(vessel_class) {
        draw_slot(view, ui, slots.get(slot_location), slot_location, center, slot_size, translation * size.x);
    }
    response.rect
}
