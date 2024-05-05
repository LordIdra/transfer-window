use eframe::{egui::{ImageButton, Pos2, Rect, Ui}, epaint};
use transfer_window_model::{components::vessel_component::{system_slot::{engine::EngineType, fuel_tank::FuelTankType, weapon::WeaponType, Slot, SlotLocation}, VesselClass}, storage::entity_allocator::Entity};

use crate::{events::Event, game::{overlay::slot_textures::TexturedSlot, Scene}, styles};

use super::{tooltips::show_tooltip, util::{compute_slot_locations, compute_slot_size}};

/// With respect to the size of the slot
const SLOT_SELECTOR_HEIGHT_PROPORTION: f32 = 0.5;
/// Absolute additional offset
const SLOT_SELECTOR_HEIGHT_OFFSET: f32 = 40.0;
/// Space between slot centers, not slot edges
const SLOT_SELECTOR_SPACING: f32 = 65.0;
const SLOT_SELECTOR_SIZE: f32 = 60.0;



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
            let image_button = ImageButton::new(view.resources.texture_image(self.texture.as_str()));
            let response = ui.add(image_button);
            let clicked = response.clicked();
            response.on_hover_ui(|ui| { 
                show_tooltip(ui, &self.slot);
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
                    let texture = type_.texture().to_string();
                    let slot = Slot::new_weapon(type_);
                    selectors.push(SlotSelector::new(texture, slot));
                }
            }

            Slot::FuelTank(_) => {
                selectors.push(SlotSelector::new("clear-slot".to_string(), Slot::FuelTank(None)));
                for type_ in FuelTankType::TYPES {
                    let texture = type_.texture().to_string();
                    let slot = Slot::new_fuel_tank(type_);
                    selectors.push(SlotSelector::new(texture, slot));
                }
            }

            Slot::Engine(_) => {
                selectors.push(SlotSelector::new("clear-slot".to_string(), Slot::Engine(None)));
                for type_ in EngineType::TYPES {
                    let texture = type_.texture().to_string();
                    let slot = Slot::new_engine(type_);
                    selectors.push(SlotSelector::new(texture, slot));
                }
            }
        };

        SlotEditor { entity, vessel_class, location, selectors }
    }

    pub fn draw(&self, view: &Scene, ui: &mut Ui, slot_center: Pos2, scalar: f32, events: &mut Vec<Event>) {
        let slot_translation = scalar * compute_slot_locations(self.vessel_class)
            .get(&self.location)
            .expect("Slot editor location does not exist");
        let first_slot_selector_position = slot_center + epaint::vec2(
            slot_translation - SLOT_SELECTOR_SPACING * (self.selectors.len() - 1) as f32 / 2.0, 
            -SLOT_SELECTOR_HEIGHT_OFFSET - SLOT_SELECTOR_HEIGHT_PROPORTION * compute_slot_size(self.vessel_class) * scalar);

        styles::SlotSelector::apply(ui);

        for (i, selector) in self.selectors.iter().enumerate() {
            if selector.draw(view, ui, i, first_slot_selector_position) {
                events.push(Event::SetSlot { entity: self.entity, slot_location: self.location, slot: selector.slot.clone() });
            }
        }
    }
}