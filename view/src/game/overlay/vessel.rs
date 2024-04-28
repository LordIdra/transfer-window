use eframe::{egui::{Align2, Context, Window}, epaint};
use transfer_window_model::{components::vessel_component::system_slot::SlotLocation, storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::Scene};

use self::{slot_editor::SlotEditor, vessel_editor::draw_vessel_editor};

mod slot_editor;
mod util;
mod vessel_editor;

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

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
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
            let vessel_class = vessel_component.class();
            let rect = draw_vessel_editor(view, context, ui, vessel_class, vessel_component.get_slots());
            let center = rect.center();
            let scalar = rect.size().x;

            // Vessel editor is queried again to satisfy borrow checker (draw_editor needs mutable access)
            let vessel_editor = view.vessel_editor.as_ref().unwrap();
            if let Some(location) = vessel_editor.slot_editor {
                let slot = vessel_component.get_slots().get(location);
                SlotEditor::new(vessel_editor.entity, vessel_class, location, slot).draw(view, ui, center, scalar, events);
            }
        });
}