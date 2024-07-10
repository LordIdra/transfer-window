use eframe::egui::{Align2, Color32, Id, LayerId, Order, Pos2, Ui, Window};
use eframe::epaint;
use transfer_window_model::components::vessel_component::class::VesselClass;
use transfer_window_model::components::vessel_component::engine::EngineType;
use transfer_window_model::components::vessel_component::fuel_tank::FuelTankType;
use transfer_window_model::components::vessel_component::torpedo_launcher::TorpedoLauncherType;
use transfer_window_model::components::vessel_component::torpedo_storage::TorpedoStorageType;
use transfer_window_model::storage::entity_allocator::Entity;

use super::tooltips::{
    show_tooltip_engine, show_tooltip_fuel_tank, show_tooltip_torpedo_launcher,
    show_tooltip_torpedo_storage, TooltipFn,
};
use super::util::{compute_slot_locations, SLOT_SIZE};
use super::SlotType;
use crate::game::events::{ModelEvent, ViewEvent};
use crate::game::overlay::slot_textures::TexturedSlot;
use crate::game::overlay::widgets::custom_image_button::CustomCircularImageButton;
use crate::game::View;
use crate::styles;

/// With respect to the size of the slot
const SLOT_SELECTOR_HEIGHT_PROPORTION: f32 = 0.6;
const SLOT_SELECTOR_SIZE: f32 = 80.0;

struct ShipSlotEditorItem {
    texture: &'static str,
    add_on_click: ModelEvent,
    show_tooltip: TooltipFn,
}

impl ShipSlotEditorItem {
    fn new(texture: &'static str, add_on_click: ModelEvent, show_tooltip: TooltipFn) -> Self {
        Self {
            texture,
            add_on_click,
            show_tooltip,
        }
    }

    pub fn draw(&self, view: &View, ui: &mut Ui) -> bool {
        let slot_selector_size = epaint::Vec2::splat(SLOT_SELECTOR_SIZE);
        let button = CustomCircularImageButton::new(view, self.texture, SLOT_SELECTOR_SIZE + 10.0)
            .with_normal_color(Color32::from_rgba_premultiplied(40, 40, 40, 240))
            .with_hover_color(Color32::from_rgba_premultiplied(70, 70, 70, 255))
            .with_padding(8.0);
        let response = ui.add_sized(slot_selector_size, button);
        let clicked = response.clicked();
        if clicked {
            view.add_model_event(self.add_on_click.clone());
        }
        styles::DefaultWindow::apply(&view.context);
        response.on_hover_ui(|ui| {
            (self.show_tooltip)(view, ui);
        });
        styles::SlotSelectorWindow::apply(&view.context);
        clicked
    }
}

pub struct ShipSlotEditor {
    vessel_class: VesselClass,
    selectors: Vec<ShipSlotEditorItem>,
}

impl ShipSlotEditor {
    pub fn new(entity: Entity, vessel_class: VesselClass, type_: SlotType) -> Self {
        let mut selectors = vec![];
        match type_ {
            SlotType::FuelTank => {
                let add_on_click = ModelEvent::SetFuelTank {
                    entity,
                    type_: None,
                };
                let tooltip = show_tooltip_fuel_tank(None);
                selectors.push(ShipSlotEditorItem::new("clear-slot", add_on_click, tooltip));
                for fuel_tank in FuelTankType::ship_types() {
                    let texture = fuel_tank.texture();
                    let add_on_click = ModelEvent::SetFuelTank {
                        entity,
                        type_: Some(fuel_tank),
                    };
                    let tooltip = show_tooltip_fuel_tank(Some(fuel_tank));
                    selectors.push(ShipSlotEditorItem::new(texture, add_on_click, tooltip));
                }
            }

            SlotType::Engine => {
                let add_on_click = ModelEvent::SetEngine {
                    entity,
                    type_: None,
                };
                let tooltip = show_tooltip_engine(None);
                selectors.push(ShipSlotEditorItem::new("clear-slot", add_on_click, tooltip));
                for engine in EngineType::ship_types() {
                    let texture = engine.texture();
                    let add_on_click = ModelEvent::SetEngine {
                        entity,
                        type_: Some(engine),
                    };
                    let tooltip = show_tooltip_engine(Some(engine));
                    selectors.push(ShipSlotEditorItem::new(texture, add_on_click, tooltip));
                }
            }

            SlotType::TorpedoStorage => {
                let add_on_click = ModelEvent::SetTorpedoStorage {
                    entity,
                    type_: None,
                };
                let tooltip = show_tooltip_torpedo_storage(None);
                selectors.push(ShipSlotEditorItem::new("clear-slot", add_on_click, tooltip));
                for torpedo_storage in TorpedoStorageType::ship_types() {
                    let texture = torpedo_storage.texture();
                    let add_on_click = ModelEvent::SetTorpedoStorage {
                        entity,
                        type_: Some(torpedo_storage),
                    };
                    let tooltip = show_tooltip_torpedo_storage(Some(torpedo_storage));
                    selectors.push(ShipSlotEditorItem::new(texture, add_on_click, tooltip));
                }
            }

            SlotType::TorpedoLauncher => {
                let add_on_click = ModelEvent::SetTorpedoLauncher {
                    entity,
                    type_: None,
                };
                let tooltip = show_tooltip_torpedo_launcher(None);
                selectors.push(ShipSlotEditorItem::new("clear-slot", add_on_click, tooltip));
                for torpedo_launcher in TorpedoLauncherType::ship_types() {
                    let texture = torpedo_launcher.texture();
                    let add_on_click = ModelEvent::SetTorpedoLauncher {
                        entity,
                        type_: Some(torpedo_launcher),
                    };
                    let tooltip = show_tooltip_torpedo_launcher(Some(torpedo_launcher));
                    selectors.push(ShipSlotEditorItem::new(texture, add_on_click, tooltip));
                }
            }
        };

        ShipSlotEditor {
            vessel_class,
            selectors,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        view: &View,
        vessel_name: &str,
        type_: SlotType,
        slot_center: Pos2,
        scalar: f32,
    ) {
        let slot_translation = scalar
            * *compute_slot_locations(self.vessel_class)
                .get(&type_)
                .expect("Slot editor location does not exist");
        let slot_selector_center = slot_center
            + slot_translation
            + epaint::vec2(0.0, -SLOT_SELECTOR_HEIGHT_PROPORTION * SLOT_SIZE * scalar);
        let mut should_close = false;
        let name = format!("Slot selector - {vessel_name} - {type_:?}");
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
