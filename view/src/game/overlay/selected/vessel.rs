use eframe::{egui::{Align2, Button, Color32, Context, Grid, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::{system_slot::System, VesselComponent}, Model};

use crate::game::{overlay::{vessel_editor::VesselEditor, widgets::draw_filled_bar}, underlay::selected::Selected, Scene};

fn draw_fuel(ui: &mut Ui, vessel_component: &VesselComponent) {
    let remaining_fuel = vessel_component.fuel_litres();
    let max_fuel = vessel_component.max_fuel_litres();
    let fuel_proportion = (remaining_fuel / max_fuel) as f32;
    ui.label("Fuel");
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::WHITE, Color32::DARK_GRAY, fuel_proportion);
    ui.label(format!("{} / {} L", remaining_fuel.round(), max_fuel));
    ui.end_row();
}

fn draw_dv(ui: &mut Ui, vessel_component: &VesselComponent) {
    let Some(remaining_dv) = vessel_component.dv() else { 
        return;
    };

    let Some(max_dv) = vessel_component.max_dv() else { 
        return;
    };

    let dv_proportion = (remaining_dv / max_dv) as f32;
    ui.label("ΔV");
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::WHITE, Color32::DARK_GRAY, dv_proportion);
    ui.label(format!("{} / {} m/s", remaining_dv.round(), max_dv.round()));
    ui.end_row();
}

fn draw_torpedoes(ui: &mut Ui, vessel_component: &VesselComponent) {
    let mut max_torpedoes = 0;
    let mut torpedoes = 0;
    for weapon in vessel_component.slots().weapon_slots() {
        if let Some(weapon) = weapon.1 {
            let torpedo = weapon.type_().as_torpedo();
            max_torpedoes += torpedo.max_stockpile();
            torpedoes += torpedo.stockpile();
        }
    }

    if max_torpedoes == 0 {
        return;
    }

    let dv_proportion = torpedoes as f32 / max_torpedoes as f32;
    ui.label("Torpedoes");
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::WHITE, Color32::DARK_GRAY, dv_proportion);
    ui.label(format!("{torpedoes} / {max_torpedoes}"));
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
            Grid::new("Vessel bars grid").show(ui, |ui| {
                if !vessel_component.slots().fuel_tanks().is_empty() {
                    draw_fuel(ui, vessel_component);
                    draw_dv(ui, vessel_component);
                }
                draw_torpedoes(ui, vessel_component);
            });

            if vessel_component.can_edit_ever() {
                let button = Button::new("Edit");
                if ui.add_enabled(model.can_edit(entity), button).clicked() {
                    view.vessel_editor = Some(VesselEditor::new(entity));
                }
            }
        });
}
