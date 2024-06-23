use eframe::egui::{Color32, Pos2, Rect, RichText, Rounding, Stroke, Ui};
use transfer_window_model::{components::vessel_component::station::{DockingPortLocation, Station}, storage::entity_allocator::Entity};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::{explorer::vessel_normal_circle_color, vessel_editor::VesselEditor, widgets::{buttons::{draw_edit_vessel, draw_undock}, custom_image_button::CustomCircularImageButton, labels::draw_subtitle, util::advance_cursor_to}}, selected::Selected, util::vessel_texture, View}, styles};

use super::draw_resources_grid;

fn draw_controls(view: &View, station: Entity, entity: Entity, ui: &mut Ui) {
    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);

        if draw_undock(view, ui) {
            view.add_model_event(ModelEvent::Undock { station, entity });
            view.add_view_event(ViewEvent::SetCameraFocus(entity));
            view.add_view_event(ViewEvent::SetSelected(Selected::Vessel(entity)));
        }

        if draw_edit_vessel(view, ui) {
            let vessel_editor = Some(VesselEditor::new(entity));
            view.add_view_event(ViewEvent::SetVesselEditor(vessel_editor));
        }
    });
}

fn draw_header(ui: &mut Ui, docked_entity: &Option<Entity>, view: &View, location: DockingPortLocation) {
    ui.horizontal(|ui| {
        let (texture, color) = match docked_entity {
            Some(docked_entity) => {
                let faction = view.model.vessel_component(*docked_entity).faction();
                let texture = vessel_texture(view.model.vessel_component(*docked_entity));
                let color = vessel_normal_circle_color(faction);
                (texture, color)
            }
            None => {
                let texture = "dock";
                let color = Color32::from_rgb(100, 100, 100);
                (texture, color)
            }
        };

        let button = CustomCircularImageButton::new(view, texture, 24.0)
            .with_padding(4.0)
            .with_margin(2.0)
            .with_normal_color(color)
            .with_hover_color(color)
            .with_pointer(false);
        let text = RichText::new(location.name()).size(15.0).monospace();

        advance_cursor_to(ui, 0.0);
        ui.add(button);
        ui.label(text);

        if let Some(docked_entity) = docked_entity {
            let name = view.model.name_component(*docked_entity).name();
            ui.label(RichText::new("-").size(14.0));
            ui.label(RichText::new(name).size(14.0).monospace().strong());
        };
    });
}

pub fn draw_docking(view: &View, ui: &mut Ui, station_entity: Entity, station: &Station) {
    draw_subtitle(ui, "Docking ports");
    for (location, docked_entity) in station.docking_ports() {
        draw_header(ui, docked_entity, view, *location);

        let Some(docked_entity) = docked_entity else {
            continue;
        };

        ui.horizontal(|ui| {
            let rect = ui.vertical(|ui| {
                ui.add_space(-5.0);
                ui.horizontal(|ui| {
                    advance_cursor_to(ui, 22.0);
                    draw_controls(view, station_entity, *docked_entity, ui);
                });
                ui.horizontal(|ui| {
                    advance_cursor_to(ui, 22.0);
                    draw_resources_grid(ui, view.model.vessel_component(*docked_entity), &view.model.name_component(*docked_entity).name());
                });
                ui.add_space(10.0);
            }).response.rect;

            let faction = view.model.vessel_component(*docked_entity).faction();
            let color = vessel_normal_circle_color(faction);
            let top_left = Pos2::new(17.0, rect.top() + 5.0);
            let bottom_right = Pos2::new(22.0, rect.bottom() - 10.0);
            let line_rect = Rect::from_min_max(top_left, bottom_right);
            ui.painter_at(line_rect).rect(line_rect, Rounding::same(2.0), color, Stroke::NONE);
            
        });
    }
}
