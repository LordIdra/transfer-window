use eframe::{egui::{style::WidgetVisuals, Color32, ImageButton, Pos2, Rect, RichText, Rounding, Stroke, Ui}, epaint};
use transfer_window_model::{components::vessel_component::{system_slot::{engine::{Engine, EngineType}, fuel_tank::{FuelTank, FuelTankType}, weapon::{Weapon, WeaponType}, Slot, SlotLocation, System}, VesselClass}, storage::entity_allocator::Entity};

use crate::{events::Event, game::Scene, icons::ICON_WAREHOUSE};

use super::util::{get_slot_locations, get_slot_size, TexturedSlot};

/// With respect to the size of the slot
const SLOT_SELECTOR_HEIGHT_PROPORTION: f32 = 0.5;
/// Absolute additional offset
const SLOT_SELECTOR_HEIGHT_OFFSET: f32 = 40.0;
/// Space between slot centers, not slot edges
const SLOT_SELECTOR_SPACING: f32 = 65.0;
const SLOT_SELECTOR_SIZE: f32 = 60.0;

fn show_tooltip_weapon(ui: &mut Ui, weapon: &Option<Weapon>) {
    let Some(weapon) = weapon else {
        ui.label("None");
        return;
    };

    let name = match weapon.get_type() {
        WeaponType::Torpedo => "Torpedo",
    };

    ui.label(name);
}

fn show_tooltip_fuel_tank(ui: &mut Ui, fuel_tank: &Option<FuelTank>) {
    let Some(fuel_tank) = fuel_tank else {
        ui.label("None");
        return;
    };

    let name = match fuel_tank.get_type() {
        FuelTankType::Small => "Small Fuel Tank",
        FuelTankType::Medium => "Medium Fuel Tank",
        FuelTankType::Large => "Large Fuel Tank",
    };

    ui.label(name);
    ui.label(RichText::new("BRUH ".to_string() + ICON_WAREHOUSE));
}

fn show_tooltip_engine(ui: &mut Ui, engine: &Option<Engine>) {
    let Some(engine) = engine else {
        ui.label("None");
        return;
    };

    let name = match engine.get_type() {
        EngineType::Efficient => "Efficient",
        EngineType::HighThrust => "High Thrust",
    };

    ui.label(name);
}

fn show_tooltip(ui: &mut Ui, slot: &Slot) {
    match slot {
        Slot::Weapon(weapon) => show_tooltip_weapon(ui, weapon),
        Slot::FuelTank(fuel_tank) => show_tooltip_fuel_tank(ui, fuel_tank),
        Slot::Engine(engine) => show_tooltip_engine(ui, engine),
    };
}

/// 1 individual item that could be equipped in the slot
struct SlotSelector {
    texture: String,
    slot: Slot,
}

impl SlotSelector {
    pub fn new(texture: String, slot: Slot) -> Self {
        Self { texture, slot }
    }

    pub fn draw(&self, view: &Scene, ui: &mut Ui, index: usize, first_slot_selector_position: Pos2) -> bool {
        let translation_x = index as f32 * SLOT_SELECTOR_SPACING;
        let slot_selector_size = epaint::Vec2::splat(SLOT_SELECTOR_SIZE);
        let slot_selector_center = first_slot_selector_position + epaint::vec2(translation_x, 0.0);
        let slot_selector_rect = Rect::from_center_size(slot_selector_center, slot_selector_size);
        ui.allocate_ui_at_rect(slot_selector_rect, |ui| {
            let image_button = ImageButton::new(view.resources.get_texture_image(self.texture.as_str()));
            let response = ui.add(image_button);
            let clicked = response.clicked();
            response.on_hover_ui(|ui| { 
                show_tooltip(ui, &self.slot)
            });
            clicked
        }).inner
    }
}

/// All the selectors together
pub struct SlotEditor {
    entity: Entity,
    vessel_class: VesselClass,
    location: SlotLocation,
    selectors: Vec<SlotSelector>,
}

impl SlotEditor {
    pub fn new(entity: Entity, vessel_class: VesselClass, location: SlotLocation, slot: &Slot) -> Self {
        let mut selectors = vec![];
        match slot {
            Slot::Weapon(_) => {
                selectors.push(SlotSelector::new("clear-slot".to_string(), Slot::Weapon(None)));
                for type_ in WeaponType::TYPES {
                    let texture = type_.get_texture().to_string();
                    let slot = Slot::Weapon(Some(Weapon::new(type_)));
                    selectors.push(SlotSelector::new(texture, slot));
                }
            }

            Slot::FuelTank(_) => {
                selectors.push(SlotSelector::new("clear-slot".to_string(), Slot::FuelTank(None)));
                for type_ in FuelTankType::TYPES {
                    let texture = type_.get_texture().to_string();
                    let slot = Slot::FuelTank(Some(FuelTank::new(type_)));
                    selectors.push(SlotSelector::new(texture, slot));
                }
            }

            Slot::Engine(_) => {
                selectors.push(SlotSelector::new("clear-slot".to_string(), Slot::Engine(None)));
                for type_ in EngineType::TYPES {
                    let texture = type_.get_texture().to_string();
                    let slot = Slot::Engine(Some(Engine::new(type_)));
                    selectors.push(SlotSelector::new(texture, slot));
                }
            }
        };

        SlotEditor { entity, vessel_class, location, selectors }
    }

    pub fn draw(&self, view: &Scene, ui: &mut Ui, slot_center: Pos2, scalar: f32, events: &mut Vec<Event>) {
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

        let slot_translation = scalar * get_slot_locations(self.vessel_class)
            .get(&self.location)
            .expect("Slot editor location does not exist");
        let first_slot_selector_position = slot_center + epaint::vec2(
            slot_translation - SLOT_SELECTOR_SPACING * (self.selectors.len() - 1) as f32 / 2.0, 
            -SLOT_SELECTOR_HEIGHT_OFFSET - SLOT_SELECTOR_HEIGHT_PROPORTION * get_slot_size(self.vessel_class) * scalar);

        for (i, selector) in self.selectors.iter().enumerate() {
            if selector.draw(view, ui, i, first_slot_selector_position) {
                events.push(Event::SetSlot { entity: self.entity, location: self.location, slot: selector.slot.clone() });
            }
        }
    }
}