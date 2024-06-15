use eframe::{egui::{Align, Align2, Layout, RichText, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::system_slot::SlotLocation, storage::entity_allocator::Entity};

use crate::game::{events::ViewEvent, View};

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
fn draw_header(view: &View, ui: &mut Ui, entity: Entity) {
    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
        let name = view.model.name_component(entity).name();
        ui.label(RichText::new(name).strong().size(32.0));
    });
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update vessel editor");
    let Some(vessel_editor) = view.vessel_editor.clone() else {
        return;
    };

    if !view.model.can_edit(vessel_editor.entity) {
        view.add_view_event(ViewEvent::SetVesselEditor(None));
        return;
    }

    let vessel_name = view.model.name_component(vessel_editor.entity).name();
    
    Window::new("Vessel editor - ".to_string() + &vessel_name)
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_header(view, ui, vessel_editor.entity);
        let rect = draw_vessel_editor(view, ui, &vessel_name, vessel_editor.entity);
        let center = rect.center();
        let scalar = rect.size().x;

        if let Some(slot_location) = vessel_editor.slot_editor {
            let vessel_component = view.model.vessel_component(vessel_editor.entity);
            let slot = vessel_component.slots().get(slot_location);
            let vessel_class = view.model.vessel_component(vessel_editor.entity).class();
            SlotEditor::new(vessel_editor.entity, vessel_class, slot_location, slot).draw(view, &vessel_name, slot_location, center, scalar);
        }
    });
}