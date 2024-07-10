use eframe::egui::{Color32, Grid, Pos2, Rect, RichText, Rounding, Stroke, Ui};
use transfer_window_model::components::vessel_component::docking::{
    ContinuousResourceTransfer, DiscreteResourceTransfer, DockingPort, DockingPortLocation,
    ResourceTransferDirection,
};
use transfer_window_model::storage::entity_allocator::Entity;

use super::{draw_dv, draw_fuel, draw_torpedoes};
use crate::game::events::{ModelEvent, ViewEvent};
use crate::game::overlay::explorer::vessel_normal_circle_color;
use crate::game::overlay::vessel_editor::VesselEditor;
use crate::game::overlay::widgets::buttons::{draw_edit_vessel, draw_undock};
use crate::game::overlay::widgets::custom_image_button::CustomCircularImageButton;
use crate::game::overlay::widgets::labels::{draw_subtitle, draw_value};
use crate::game::overlay::widgets::util::{
    advance_cursor_to, should_draw_dv, should_draw_fuel, should_draw_torpedoes,
};
use crate::game::selected::Selected;
use crate::game::util::{format_time, vessel_texture};
use crate::game::View;
use crate::styles;

#[allow(clippy::too_many_arguments)]
fn draw_fuel_transfer_button(
    view: &View,
    ui: &mut Ui,
    station_entity: Entity,
    location: DockingPortLocation,
    direction: ResourceTransferDirection,
    texture: &str,
    is_transferring: bool,
    is_other_transferring: bool,
    can_transfer: bool,
) {
    if is_transferring {
        let button = CustomCircularImageButton::new(view, "transfer-cancel", 14.0);
        if ui.add(button).clicked() {
            view.add_model_event(ModelEvent::StopFuelTransfer {
                station: station_entity,
                location,
            });
        }
    } else {
        let enabled = can_transfer && !is_other_transferring;
        let button = CustomCircularImageButton::new(view, texture, 14.0).with_enabled(enabled);
        if ui.add_enabled(enabled, button).clicked() {
            view.add_model_event(ModelEvent::StartFuelTransfer {
                station: station_entity,
                location,
                direction,
            });
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_torpedo_transfer_button(
    view: &View,
    ui: &mut Ui,
    station_entity: Entity,
    location: DockingPortLocation,
    direction: ResourceTransferDirection,
    texture: &str,
    is_transferring: bool,
    is_other_transferring: bool,
    can_transfer: bool,
) {
    if is_transferring {
        let button = CustomCircularImageButton::new(view, "transfer-cancel", 14.0);
        if ui.add(button).clicked() {
            view.add_model_event(ModelEvent::StopTorpedoTransfer {
                station: station_entity,
                location,
            });
        }
    } else {
        let enabled = can_transfer && !is_other_transferring;
        let button = CustomCircularImageButton::new(view, texture, 14.0).with_enabled(enabled);
        if ui.add_enabled(enabled, button).clicked() {
            view.add_model_event(ModelEvent::StartTorpedoTransfer {
                station: station_entity,
                location,
                direction,
            });
        }
    }
}

fn draw_resources_grid(
    view: &View,
    ui: &mut Ui,
    station_entity: Entity,
    docked_entity: Entity,
    location: DockingPortLocation,
) {
    let vessel_component = view.model.vessel_component(docked_entity);
    let name = view.model.name_component(docked_entity).name();
    Grid::new("Vessel resource grid ".to_string() + &name).show(ui, |ui| {
        if should_draw_dv(vessel_component) {
            ui.horizontal(|_| ());
            draw_dv(ui, vessel_component, Color32::WHITE);
            ui.end_row();
        }

        if should_draw_fuel(vessel_component) {
            let transfer =
                view.model.docking_port(station_entity, location).docked_vessel().fuel_transfer();
            let direction = transfer.map(ContinuousResourceTransfer::direction);
            let is_transfer_from = direction.is_some_and(|direction| direction.is_from_docked());
            let is_transfer_to = direction.is_some_and(|direction| direction.is_to_docked());
            let can_transfer_from_docked =
                view.model.can_transfer_fuel_from_docked(station_entity, location);
            let can_transfer_to_docked =
                view.model.can_transfer_fuel_to_docked(station_entity, location);

            ui.horizontal(|ui| {
                draw_fuel_transfer_button(
                    view,
                    ui,
                    station_entity,
                    location,
                    ResourceTransferDirection::FromDocked,
                    "transfer-from",
                    is_transfer_from,
                    is_transfer_to,
                    can_transfer_from_docked,
                );
                draw_fuel_transfer_button(
                    view,
                    ui,
                    station_entity,
                    location,
                    ResourceTransferDirection::ToDocked,
                    "transfer-to",
                    is_transfer_to,
                    is_transfer_from,
                    can_transfer_to_docked,
                );
            });
            draw_fuel(ui, vessel_component, Color32::WHITE);
            ui.end_row();
        }

        if should_draw_torpedoes(vessel_component) {
            let transfer = view
                .model
                .docking_port(station_entity, location)
                .docked_vessel()
                .torpedo_transfer();
            let direction = transfer.map(DiscreteResourceTransfer::direction);
            let is_transfer_from = direction.is_some_and(|direction| direction.is_from_docked());
            let is_transfer_to = direction.is_some_and(|direction| direction.is_to_docked());
            let can_transfer_from_docked =
                view.model.can_transfer_torpedoes_from_docked(station_entity, location);
            let can_transfer_to_docked =
                view.model.can_transfer_torpedoes_to_docked(station_entity, location);

            ui.horizontal(|ui| {
                draw_torpedo_transfer_button(
                    view,
                    ui,
                    station_entity,
                    location,
                    ResourceTransferDirection::FromDocked,
                    "transfer-from",
                    is_transfer_from,
                    is_transfer_to,
                    can_transfer_from_docked,
                );
                draw_torpedo_transfer_button(
                    view,
                    ui,
                    station_entity,
                    location,
                    ResourceTransferDirection::ToDocked,
                    "transfer-to",
                    is_transfer_to,
                    is_transfer_from,
                    can_transfer_to_docked,
                );
            });
            draw_torpedoes(ui, vessel_component, Color32::WHITE);
            if let Some(transfer) = transfer {
                draw_value(
                    ui,
                    &format!("Transfer T-{}", format_time(transfer.time_to_next())),
                );
            }
            ui.end_row();
        }
    });
}

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

fn draw_header(
    ui: &mut Ui,
    docking_port: &DockingPort,
    view: &View,
    location: DockingPortLocation,
) {
    ui.horizontal(|ui| {
        let (texture, color) = if docking_port.has_docked_vessel() {
            let docked_entity = docking_port.docked_vessel().entity();
            let faction = view.model.vessel_component(docked_entity).faction();
            let texture = vessel_texture(view.model.vessel_component(docked_entity));
            let color = vessel_normal_circle_color(faction);
            (texture, color)
        } else {
            let texture = "dock";
            let color = Color32::from_rgb(100, 100, 100);
            (texture, color)
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

        if docking_port.has_docked_vessel() {
            let name = view.model.name_component(docking_port.docked_vessel().entity()).name();
            ui.label(RichText::new("-").size(14.0));
            ui.label(RichText::new(name).size(14.0).monospace().strong());
        };
    });
}

pub fn draw_docking(view: &View, ui: &mut Ui, station_entity: Entity) {
    draw_subtitle(ui, "Docking ports");
    for (location, docking_port) in
        view.model.vessel_component(station_entity).docking_ports().unwrap()
    {
        draw_header(ui, docking_port, view, *location);

        if !docking_port.has_docked_vessel() {
            continue;
        };
        let docked_entity = docking_port.docked_vessel().entity();

        ui.horizontal(|ui| {
            let rect = ui
                .vertical(|ui| {
                    ui.add_space(-5.0);
                    ui.horizontal(|ui| {
                        advance_cursor_to(ui, 22.0);
                        draw_controls(view, station_entity, docked_entity, ui);
                    });
                    ui.horizontal(|ui| {
                        advance_cursor_to(ui, 22.0);
                        draw_resources_grid(view, ui, station_entity, docked_entity, *location);
                    });
                    ui.add_space(10.0);
                })
                .response
                .rect;

            let faction = view.model.vessel_component(docked_entity).faction();
            let color = vessel_normal_circle_color(faction);
            let top_left = Pos2::new(17.0, rect.top() + 5.0);
            let bottom_right = Pos2::new(22.0, rect.bottom() - 10.0);
            let line_rect = Rect::from_min_max(top_left, bottom_right);
            ui.painter_at(line_rect).rect(line_rect, Rounding::same(2.0), color, Stroke::NONE);
        });
    }
}
