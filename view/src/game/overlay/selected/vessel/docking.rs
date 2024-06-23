use eframe::egui::{RichText, Ui};
use transfer_window_model::components::vessel_component::station::Station;

use crate::game::{overlay::{explorer::vessel_normal_circle_color, widgets::{custom_image::CustomImage, custom_image_button::CustomCircularImageButton, labels::draw_subtitle, util::advance_cursor_to}}, util::vessel_texture, View};

pub fn draw_docking(view: &View, ui: &mut Ui, station: &Station) {
    draw_subtitle(ui, "Docking ports");
    for (location, docked_entity) in station.docking_ports() {
        let texture = match docked_entity {
            Some(_) => "docked-some",
            None => "docked-none",
        };
        let image = CustomImage::new(view, texture, 16.0);

        ui.horizontal(|ui| {
            advance_cursor_to(ui, 0.0);
            ui.add(image);
            ui.label(RichText::new(location.name()).size(15.0).monospace());
        });

        ui.horizontal(|ui| {
            let Some(docked_entity) = docked_entity else {
                return;
            };

            let faction = view.model.vessel_component(*docked_entity).faction();
            let name = view.model.name_component(*docked_entity).name();
            let texture = vessel_texture(view.model.vessel_component(*docked_entity));
            let color = vessel_normal_circle_color(faction);
            let button = CustomCircularImageButton::new(view, texture, 24.0)
                .with_padding(4.0)
                .with_margin(2.0)
                .with_normal_color(color)
                .with_hover_color(color)
                .with_pointer(false);

            advance_cursor_to(ui, -4.0);
            ui.add(CustomImage::new(view, "explorer-corner", 24.0));
            advance_cursor_to(ui, 22.0);
            ui.add(button);
            advance_cursor_to(ui, 48.0);
            ui.label(RichText::new(name).size(14.0).strong());
        });
    }
}