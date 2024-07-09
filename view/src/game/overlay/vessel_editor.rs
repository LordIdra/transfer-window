use eframe::{egui::{Align, Align2, Color32, Layout, RichText, Ui, Window}, epaint};
use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::{events::ViewEvent, View};

use self::{slot_editor::ShipSlotEditor, ship::draw_vessel_editor};

use super::widgets::custom_image_button::CustomCircularImageButton;

mod slot_editor;
mod tooltips;
mod util;
mod ship;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SlotType {
    Engine,
    FuelTank,
    TorpedoStorage,
    TorpedoLauncher,
}

#[derive(Debug, Clone)]
pub struct VesselEditor {
    entity: Entity,
    slot_editor: Option<SlotType>,
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

    if !view.model.docked(vessel_editor.entity) {
        view.add_view_event(ViewEvent::SetVesselEditor(None));
        return;
    }

    let vessel_name = view.model.name_component(vessel_editor.entity).name();
    
    Window::new("Vessel editor - ".to_string() + &vessel_name)
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {

        let rect = ui.horizontal_top(|ui| {
            let rect = ui.vertical(|ui| {
                draw_header(view, ui, vessel_editor.entity);
                draw_vessel_editor(view, ui, &vessel_name, vessel_editor.entity)
            }).inner;

            let button = CustomCircularImageButton::new(view, "cancel", 36.0)
                .with_padding(12.0)
                .with_normal_color(Color32::from_rgb(60, 60, 60))
                .with_hover_color(Color32::from_rgb(80, 80, 80));
            if ui.add(button).on_hover_text("Close editor").clicked() {
                view.add_view_event(ViewEvent::SetVesselEditor(None));
            }

            rect
        }).inner;

        let center = rect.center();
        let scalar = rect.size().x;

        if let Some(slot_type) = vessel_editor.slot_editor {
            let vessel_component = view.model.vessel_component(vessel_editor.entity);
            let class = vessel_component.class();
            ShipSlotEditor::new(vessel_editor.entity, class, slot_type).draw(view, &vessel_name, slot_type, center, scalar);
        }
    });
}