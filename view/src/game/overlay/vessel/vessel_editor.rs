use eframe::egui::{self, Color32, Context, Image, ImageButton, Pos2, Rect, Response, Ui};
use transfer_window_model::components::vessel_component::{system_slot::{Slot, SlotLocation, Slots}, VesselClass};

use crate::{game::Scene, styles};

use super::util::{compute_slot_locations, compute_slot_size, TexturedSlot};

const UNDERLAY_SIZE_PROPORTION: f32 = 0.9;
const WEAPON_SLOT_COLOR: Color32 = Color32::from_rgb(212, 11, 24);
const FUEL_TANK_SLOT_COLOR: Color32 = Color32::from_rgb(240, 200, 0);
const ENGINE_SLOT_COLOR: Color32 = Color32::from_rgb(2, 192, 240);

fn compute_texture_ship_underlay(class: &VesselClass) -> &str {
    match class {
        VesselClass::Light => "ship-light",
    }
}

fn on_slot_clicked(view: &mut Scene, location: SlotLocation) {
    if let Some(vessel_editor) = &mut view.vessel_editor {
        if vessel_editor.slot_editor == Some(location) {
            vessel_editor.slot_editor = None;
        } else {
            vessel_editor.slot_editor = Some(location);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_slot_from_texture(view: &mut Scene, ui: &mut Ui, texture: &str, color: Color32, location: SlotLocation, center: Pos2, size: f32, translation: f32) {
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

fn draw_slot(view: &mut Scene, ui: &mut Ui, slot: &Slot, location: SlotLocation, center: Pos2, size: f32, translation: f32) {
    let texture = slot.texture();
    let color = match slot {
        Slot::Engine(_) => ENGINE_SLOT_COLOR,
        Slot::FuelTank(_) => FUEL_TANK_SLOT_COLOR,
        Slot::Weapon(_) => WEAPON_SLOT_COLOR,
    };

    draw_slot_from_texture(view, ui, texture, color, location, center, size, translation);
}

fn draw_ship_underlay(view: &mut Scene, context: &Context, ui: &mut Ui, class: VesselClass) -> Response {
    let texture = view.resources.texture_image(compute_texture_ship_underlay(&class));
    let size = context.screen_rect().size() * UNDERLAY_SIZE_PROPORTION;
    ui.add(Image::new(texture).fit_to_exact_size(size))
}

pub fn draw_vessel_editor(view: &mut Scene, context: &Context, ui: &mut Ui, vessel_class: VesselClass, slots: &Slots) -> Rect {
    let response = draw_ship_underlay(view, context, ui, VesselClass::Light);
    let center = response.rect.center();
    let size = response.rect.size();
    let slot_size = compute_slot_size(vessel_class) * size.x;
    for (slot_location, translation) in compute_slot_locations(vessel_class) {
        draw_slot(view, ui, slots.get(slot_location), slot_location, center, slot_size, translation * size.x);
    }
    response.rect
}
