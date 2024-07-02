use eframe::{egui::{Align2, Color32, Id, LayerId, Order, Pos2, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::ship::{ship_slot::{engine::EngineType, fuel_tank::FuelTankType, weapon::WeaponType, ShipSlot, ShipSlotLocation}, ShipClass}, storage::entity_allocator::Entity};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::{slot_textures::TexturedSlot, widgets::custom_image_button::CustomCircularImageButton}, View}, styles};

use super::{tooltips::show_tooltip, util::{compute_slot_locations, compute_slot_size}};

/// With respect to the size of the slot
const SLOT_SELECTOR_HEIGHT_PROPORTION: f32 = 0.5;
const SLOT_SELECTOR_SIZE: f32 = 50.0;



/// 1 individual item that could be equipped in the slot
struct ShipSlotSelector {
    texture: String,
    slot: ShipSlot,
}

impl ShipSlotSelector {
    pub fn new(texture: String, slot: ShipSlot) -> Self {
        Self { texture, slot }
    }

    pub fn draw(&self, view: &View, ui: &mut Ui) -> bool {
        let slot_selector_size = epaint::Vec2::splat(SLOT_SELECTOR_SIZE);        
        let button = CustomCircularImageButton::new(view, &self.texture, 60.0)
            .with_normal_color(Color32::from_rgba_premultiplied(40, 40, 40, 170))
            .with_hover_color(Color32::from_rgba_premultiplied(70, 70, 70, 220))
            .with_padding(8.0);
        let response = ui.add_sized(slot_selector_size, button);
        let clicked = response.clicked();
        styles::DefaultWindow::apply(&view.context);
        response.on_hover_ui(|ui| { 
            show_tooltip(view, ui, &self.slot);
        });
        styles::SlotSelectorWindow::apply(&view.context);
        clicked
    }
}

/// All the selectors together
pub struct ShipSlotEditor {
    entity: Entity,
    ship_class: ShipClass,
    location: ShipSlotLocation,
    selectors: Vec<ShipSlotSelector>,
}

impl ShipSlotEditor {
    pub fn new(entity: Entity, ship_class: ShipClass, location: ShipSlotLocation, slot: &ShipSlot) -> Self {
        let mut selectors = vec![];
        match slot {
            ShipSlot::Weapon(_) => {
                selectors.push(ShipSlotSelector::new("clear-slot".to_string(), ShipSlot::Weapon(None)));
                for type_ in WeaponType::types() {
                    let texture = type_.texture().to_string();
                    let slot = ShipSlot::new_weapon(type_);
                    selectors.push(ShipSlotSelector::new(texture, slot));
                }
            }

            ShipSlot::FuelTank(_) => {
                selectors.push(ShipSlotSelector::new("clear-slot".to_string(), ShipSlot::FuelTank(None)));
                for type_ in FuelTankType::types() {
                    let texture = type_.texture().to_string();
                    let slot = ShipSlot::new_fuel_tank(type_);
                    selectors.push(ShipSlotSelector::new(texture, slot));
                }
            }

            ShipSlot::Engine(_) => {
                selectors.push(ShipSlotSelector::new("clear-slot".to_string(), ShipSlot::Engine(None)));
                for type_ in EngineType::types() {
                    let texture = type_.texture().to_string();
                    let slot = ShipSlot::new_engine(type_);
                    selectors.push(ShipSlotSelector::new(texture, slot));
                }
            }
        };

        ShipSlotEditor { entity, ship_class, location, selectors }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(&self, view: &View, vessel_name: &str, slot_location: ShipSlotLocation, slot_center: Pos2, scalar: f32) {
        let slot_translation = scalar * compute_slot_locations(self.ship_class)
            .get(&self.location)
            .expect("Slot editor location does not exist");
        let slot_selector_center = slot_center + epaint::vec2(
            slot_translation, 
            -SLOT_SELECTOR_HEIGHT_PROPORTION * compute_slot_size(self.ship_class) * scalar);
        let mut should_close = false;
        let name = format!("Slot selector - {vessel_name} - {slot_location:?}");
        let id = Id::new(name.clone());
        
        styles::SlotSelectorWindow::apply(&view.context);
        Window::new(name)
                .title_bar(false)
                .resizable(false)
                .pivot(Align2::CENTER_BOTTOM)
                .current_pos(slot_selector_center)
                .movable(false)
                .id(id)
                .show(&view.context.clone(), |ui| {
            styles::SlotSelector::apply(ui);

            ui.horizontal(|ui| {
                for selector in &self.selectors {
                    if selector.draw(view, ui) {
                        view.add_model_event(ModelEvent::SetSlot { entity: self.entity, slot_location: self.location, slot: selector.slot.clone() });
                        should_close = true;
                    }
                }
            })
        });

        // Bring window to top
        // https://github.com/emilk/egui/discussions/3493
        let layer_id = LayerId::new(Order::Middle, id);
        view.context.move_to_top(layer_id);

        if should_close {
            let mut vessel_editor = view.vessel_editor.as_ref().unwrap().clone();
            vessel_editor.slot_editor = None;
            view.add_view_event(ViewEvent::SetVesselEditor(Some(vessel_editor)));
        }
    }
}