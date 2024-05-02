use eframe::{egui::{Align, Align2, Color32, Context, Layout, Pos2, Rect, Rounding, Sense, Stroke, Window}, emath::RectTransform, epaint};
use transfer_window_model::Model;

use crate::{events::Event, game::{overlay::vessel::VesselEditor, underlay::selected::Selected, Scene}};

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::Vessel(entity) = view.selected.clone() else { 
        return
    };

    Window::new("Selected vessel")
    .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            let vessel_component = model.get_vessel_component(entity);
            if !vessel_component.get_slots().get_fuel_tanks().is_empty() {
                ui.with_layout(Layout::left_to_right(Align::TOP),|ui| {
                    ui.label("Fuel");

                    let (response, painter) = ui.allocate_painter(epaint::vec2(120.0, 10.0), Sense::hover());
                    let to_screen = RectTransform::from_to(
                        Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                        response.rect,
                    );

                    let width = 114.0 * vessel_component.get_remaining_fuel_litres() / vessel_component.get_max_fuel_litres();
                    let from = to_screen.transform_pos(Pos2::new(2.0, 2.0));
                    let to = to_screen.transform_pos(Pos2::new(118.0, 8.0));
                    painter.rect(Rect::from_min_max(from, to), Rounding::same(3.0), Color32::DARK_GRAY, Stroke::NONE);

                    let from = to_screen.transform_pos(Pos2::new(2.0, 2.0));
                    let to = to_screen.transform_pos(Pos2::new(2.0 + width as f32, 8.0));
                    painter.rect(Rect::from_min_max(from, to), Rounding::same(3.0), Color32::WHITE, Stroke::NONE);

                    ui.label(format!("{} / {}", vessel_component.get_remaining_fuel_litres(), vessel_component.get_max_fuel_litres()));
                });
            }
            
            if ui.button("Edit").clicked() {
                view.vessel_editor = Some(VesselEditor::new(entity));
            }
            if ui.button("Yeet").clicked() {
                events.push(Event::Destroy { entity });
            }
        });
}
