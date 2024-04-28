use eframe::{egui::{style::WidgetVisuals, Color32, ImageButton, ImageSource, Pos2, Rect, Rounding, Stroke, Ui}, epaint};
use transfer_window_model::{components::vessel_component::{system_slot::{engine::{Engine, EngineType}, fuel_tank::{FuelTank, FuelTankType}, weapon::{Weapon, WeaponType}, Slot, SlotLocation, Slots}, VesselClass}, storage::entity_allocator::Entity};

use crate::{events::Event, game::Scene};

use super::util::{get_slot_locations, get_slot_size, TexturedSlot};

/// With respect to the size of the slot
const SLOT_SELECTOR_HEIGHT_PROPORTION: f32 = 0.5;
/// Absolute additional offset
const SLOT_SELECTOR_HEIGHT_OFFSET: f32 = 40.0;
/// Space between slot centers, not slot edges
const SLOT_SELECTOR_SPACING: f32 = 65.0;
const SLOT_SELECTOR_SIZE: f32 = 60.0;

fn draw_slot_selector(ui: &mut Ui, texture: ImageSource, first_slot_selector_position: Pos2, i: usize) -> bool {
        let translation_x = i as f32 * SLOT_SELECTOR_SPACING;
        let slot_selector_size = epaint::Vec2::splat(SLOT_SELECTOR_SIZE);
        let slot_selector_center = first_slot_selector_position + epaint::vec2(translation_x, 0.0);
        let slot_selector_rect = Rect::from_center_size(slot_selector_center, slot_selector_size);
        ui.allocate_ui_at_rect(slot_selector_rect, |ui| {
            let image_button = ImageButton::new(texture);
            ui.add(image_button).clicked()
        }).inner
}

/// Slot Selector = 1 individual item that could be equipped in the slot
/// Slot Editor = all the selectors together
/// TODO: This is poorly abstracted and will need redesign in the future to support more slot types
#[allow(clippy::too_many_arguments)]
pub fn draw(view: &Scene, ui: &mut Ui, slots: &Slots, class: VesselClass, location: SlotLocation, center: Pos2, size_x: f32, entity: Entity, events: &mut Vec<Event>) {
    ui.visuals_mut().widgets.inactive = WidgetVisuals {
        bg_fill: Color32::from_rgba_unmultiplied(40, 40, 40, 220),
        weak_bg_fill: Color32::from_rgba_unmultiplied(40, 40, 40, 220),
        bg_stroke: Stroke::NONE,
        rounding: Rounding::ZERO,
        fg_stroke: Stroke::NONE,
        expansion: 0.0,
    };

    ui.visuals_mut().widgets.hovered = WidgetVisuals {
        bg_fill: Color32::from_rgba_unmultiplied(60, 60, 60, 220),
        weak_bg_fill: Color32::from_rgba_unmultiplied(60, 60, 60, 220),
        bg_stroke: Stroke::NONE,
        rounding: Rounding::ZERO,
        fg_stroke: Stroke::NONE,
        expansion: 0.0,
    };

    ui.visuals_mut().widgets.active = WidgetVisuals {
        bg_fill: Color32::from_rgba_unmultiplied(80, 80, 80, 220),
        weak_bg_fill: Color32::from_rgba_unmultiplied(80, 80, 80, 220),
        bg_stroke: Stroke::NONE,
        rounding: Rounding::ZERO,
        fg_stroke: Stroke::NONE,
        expansion: 0.0,
    };

    let selector_count = match slots.get(location) {
        Slot::Weapon(_) => WeaponType::TYPES.len(),
        Slot::FuelTank(_) => FuelTankType::TYPES.len(),
        Slot::Engine(_) => EngineType::TYPES.len(),
    };

    let slot_translation = size_x * get_slot_locations(class)
        .get(&location)
        .expect("Slot editor location does not exist");
    let first_slot_selector_position = center + epaint::vec2(
        slot_translation - SLOT_SELECTOR_SPACING * (selector_count - 1) as f32 / 2.0, 
        -SLOT_SELECTOR_HEIGHT_OFFSET - SLOT_SELECTOR_HEIGHT_PROPORTION * get_slot_size(class) * size_x);

    match slots.get(location) {
        Slot::Weapon(_) => {
            for (i, type_) in WeaponType::TYPES.iter().enumerate() {
                let texture = view.resources.get_texture_image(type_.get_texture());
                if draw_slot_selector(ui, texture, first_slot_selector_position, i) {
                    let slot = Slot::Weapon(Some(Weapon::new(*type_)));
                    events.push(Event::SetSlot { entity, location, slot });
                }
            }
        }

        Slot::FuelTank(_) => {
            for (i, type_) in FuelTankType::TYPES.iter().enumerate() {
                let texture = view.resources.get_texture_image(type_.get_texture());
                if draw_slot_selector(ui, texture, first_slot_selector_position, i) {
                    let slot = Slot::FuelTank(Some(FuelTank::new(*type_)));
                    events.push(Event::SetSlot { entity, location, slot });
                }
            }
        }

        Slot::Engine(_) => {
            for (i, type_) in EngineType::TYPES.iter().enumerate() {
                let texture = view.resources.get_texture_image(type_.get_texture());
                if draw_slot_selector(ui, texture, first_slot_selector_position, i) {
                    let slot = Slot::Engine(Some(Engine::new(*type_)));
                    events.push(Event::SetSlot { entity, location, slot });
                }
            }
        }
    };
}