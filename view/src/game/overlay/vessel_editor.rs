use eframe::{egui::{Align, Align2, Context, Layout, RichText, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::system_slot::SlotLocation, storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::Scene};

use self::{slot_editor::SlotEditor, vessel::draw_vessel_editor};

mod slot_editor;
mod tooltips;
mod util;
mod vessel;


#[derive(Debug, Clone)]
pub struct VesselEditor {
    entity: Entity,
    slot_editor: Option<SlotLocation>,
}

impl VesselEditor {
    pub fn new(entity: Entity) -> VesselEditor {
        Self { entity, slot_editor: None }
    }
}

/// Returns whether the close button was clicked
fn draw_header(model: &Model, ui: &mut Ui, entity: Entity) {
    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
        let name = model.name_component(entity).name().to_uppercase();
        ui.label(RichText::new(name).strong().size(32.0));
    });
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Some(vessel_editor) = view.vessel_editor.clone() else {
        return;
    };

    if !model.can_edit(vessel_editor.entity) {
        view.vessel_editor = None;
        return;
    }

    let vessel_name = model.name_component(vessel_editor.entity).name();
    
    Window::new("Vessel editor - ".to_string() + &vessel_name)
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, epaint::vec2(0.0, 0.0))
            .show(context, |ui| {
        draw_header(model, ui, vessel_editor.entity);
        let vessel_component = model.vessel_component(vessel_editor.entity);
        let vessel_class = vessel_component.class();
        let rect = draw_vessel_editor(view, context, ui, &vessel_name, vessel_component);
        let center = rect.center();
        let scalar = rect.size().x;

        if let Some(slot_location) = vessel_editor.slot_editor {
            let slot = vessel_component.slots().get(slot_location);
            SlotEditor::new(vessel_editor.entity, vessel_class, slot_location, slot).draw(view, context, &vessel_name, slot_location, center, scalar, events);
        }
    });
}