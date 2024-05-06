use eframe::{egui::{Align, Align2, Context, ImageButton, Layout, Ui, Window}, epaint};
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
fn draw_header(view: &mut Scene, model: &Model, ui: &mut Ui, entity: Entity) -> bool {
    let mut should_close = false;
    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        let button = ImageButton::new(view.resources.texture_image("close"));
        if ui.add_sized(epaint::vec2(20.0, 20.0), button).clicked() {
            should_close = true;
        }
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.label(model.name_component(entity).name().to_uppercase());
        });
    });
    should_close
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
            let vessel_editor = view.vessel_editor.as_ref().unwrap().clone();
            let should_close = draw_header(view, model, ui, vessel_editor.entity);
            let vessel_component = model.vessel_component(vessel_editor.entity);
            let vessel_class = vessel_component.class();
            let rect = draw_vessel_editor(view, context, ui, vessel_class, vessel_component.slots());
            let center = rect.center();
            let scalar = rect.size().x;

            if let Some(location) = vessel_editor.slot_editor {
                let slot = vessel_component.slots().get(location);
                SlotEditor::new(vessel_editor.entity, vessel_class, location, slot).draw(view, ui, center, scalar, events);
            }

            if should_close {
                view.vessel_editor = None;
            }
        });
}