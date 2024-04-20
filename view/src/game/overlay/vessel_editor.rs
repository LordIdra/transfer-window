use std::collections::BTreeMap;

use eframe::{egui::{Align2, Context, Rect, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::{system_slot::{SlotLocation, Slots}, VesselClass}, storage::entity_allocator::Entity, Model};

use crate::game::Scene;

use self::draw::{draw_ship_underlay, draw_slot};

mod draw;

const SLOT_EDITOR_SIZE: f32 = 50.0;

pub struct VesselEditor {
    entity: Entity,
    slot_editor: Option<SlotLocation>,
}

impl VesselEditor {
    pub fn new(entity: Entity) -> VesselEditor {
        Self { entity, slot_editor: None }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}

fn get_slot_size(class: VesselClass) -> f32 {
    match class {
        VesselClass::Light => 0.113,
    }
}

fn get_slot_locations(class: VesselClass) -> BTreeMap<SlotLocation, f32> {
    match class {
        VesselClass::Light => vec![
            (SlotLocation::Front, -0.142),
            (SlotLocation::Middle, 0.177),
            (SlotLocation::Back, 0.382),
        ].into_iter().collect(),
    }
}

fn draw_editor(view: &mut Scene, context: &Context, ui: &mut Ui, class: VesselClass, slots: &Slots) -> Rect {
    let response = draw_ship_underlay(view, context, ui, VesselClass::Light);
    let center = response.rect.center();
    let size = response.rect.size();
    let slot_size = get_slot_size(class) * size.x;
    for (slot_location, translation) in get_slot_locations(class) {
        draw_slot(view, ui, slots.get(slot_location), slot_location, center, slot_size, translation * size.x);
    }
    response.rect
}


pub fn update(view: &mut Scene, model: &Model, context: &Context) {
    if view.vessel_editor.is_none() {
        return;
    };

    Window::new("Vessel editor")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            let vessel_editor = view.vessel_editor.as_ref().unwrap();
            ui.label(model.get_name_component(vessel_editor.entity).get_name().to_uppercase());
            let vessel_component = model.get_vessel_component(vessel_editor.entity);
            let class = vessel_component.class();
            let rect = draw_editor(view, context, ui, class, vessel_component.slots());
            let center = rect.center();
            let size_x = rect.size().x;

            // Vessel editor is queried again to satisfy borrow checker (draw_editor needs mutable access)
            let vessel_editor = view.vessel_editor.as_ref().unwrap();
            if let Some(location) = vessel_editor.slot_editor {
                let translation = size_x * get_slot_locations(class).get(&location).expect("Slot editor location does not exist");
                let slot_editor_center = center + epaint::vec2(translation, -0.5 * get_slot_size(class) * size_x);
                let slot_editor_rect = Rect::from_center_size(slot_editor_center, epaint::Vec2::splat(SLOT_EDITOR_SIZE));
                ui.allocate_ui_at_rect(slot_editor_rect, |ui| ui.label("bruh"));
            }
        });
}