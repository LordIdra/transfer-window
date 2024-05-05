use eframe::{egui::{Align2, Color32, Context, Grid, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::VesselComponent, Model};

use crate::game::{overlay::{vessel::VesselEditor, widgets::draw_filled_bar}, underlay::selected::Selected, Scene};

fn draw_fuel(ui: &mut Ui, vessel_component: &VesselComponent) {
    let remaining_fuel = vessel_component.remaining_fuel_litres();
    let max_fuel = vessel_component.max_fuel_litres();
    let fuel_proportion = (remaining_fuel / max_fuel) as f32;
    ui.label("Fuel");
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::WHITE, Color32::DARK_GRAY, fuel_proportion);
    ui.label(format!("{} / {}", remaining_fuel.round(), max_fuel));
    ui.end_row();
}

fn draw_dv(ui: &mut Ui, vessel_component: &VesselComponent) {
    let Some(remaining_dv) = vessel_component.remaining_dv() else { 
        return;
    };
    let Some(max_dv) = vessel_component.max_dv() else { 
        return;
    };
    let dv_proportion = (remaining_dv / max_dv) as f32;
    ui.label("Delta-V");
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::WHITE, Color32::DARK_GRAY, dv_proportion);
    ui.label(format!("{} / {}", remaining_dv.round(), max_dv.round()));
    ui.end_row();
}

pub fn update(view: &mut Scene, model: &Model, context: &Context) {
    let Selected::Vessel(entity) = view.selected.clone() else { 
        return
    };

    Window::new("Selected vessel")
    .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            let vessel_component = model.vessel_component(entity);
            if !vessel_component.slots().fuel_tanks().is_empty() {
                Grid::new("Vessel bars grid").show(ui, |ui| {
                    draw_fuel(ui, vessel_component);
                    draw_dv(ui, vessel_component);
                });
            }

            if !vessel_component.can_edit() {
                if ui.button("Edit").clicked() {
                    view.vessel_editor = Some(VesselEditor::new(entity));
                }
            }
            
        });
}
