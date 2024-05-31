use eframe::{egui::{Align2, Color32, Context, Grid, ImageButton, RichText, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::{system_slot::System, VesselComponent}, Model};

use crate::{events::Event, game::{overlay::{vessel_editor::VesselEditor, widgets::{draw_filled_bar, FilledBar}}, underlay::selected::Selected, Scene}, styles};

fn draw_fuel(ui: &mut Ui, vessel_component: &VesselComponent) {
    let remaining_fuel = vessel_component.fuel_litres();
    let max_fuel = vessel_component.max_fuel_litres();
    let fuel_proportion = (remaining_fuel / max_fuel) as f32;
    ui.label(RichText::new("Fuel").monospace().strong());
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::DARK_GRAY, vec![FilledBar::new(Color32::WHITE, fuel_proportion)]);
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
    ui.label(RichText::new("ΔV").monospace().strong());
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::DARK_GRAY, vec![FilledBar::new(Color32::WHITE, dv_proportion)]);
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
    ui.label(RichText::new("Torpedoes").monospace().strong());
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::DARK_GRAY, vec![FilledBar::new(Color32::WHITE, dv_proportion)]);
    ui.label(format!("{torpedoes} / {max_torpedoes}"));
    ui.end_row();
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::Vessel(entity) = view.selected.clone() else { 
        return
    };

    let vessel_component = model.vessel_component(entity);
    let name = model.name_component(entity).name().to_uppercase();

    Window::new("Selected vessel ".to_string() + name.as_str())
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label(RichText::new(name).size(20.0).strong().monospace());

            let add_edit_button = vessel_component.can_edit_ever();
            let add_cancel_burn_button = model.path_component(entity).current_segment().is_burn();
            let add_cancel_guidance_button = model.path_component(entity).current_segment().is_guidance();
            if add_edit_button || add_cancel_burn_button || add_cancel_guidance_button {
                ui.horizontal(|ui| {
                    styles::SelectedMenuButton::apply(ui);
                    ui.set_height(36.0);

                    if add_edit_button {
                        let button = ImageButton::new(view.resources.texture_image("edit"));
                        if ui.add_enabled(model.can_edit(entity), button).on_hover_text("Edit").clicked() {
                            view.vessel_editor = Some(VesselEditor::new(entity));
                        }
                    }

                    if add_cancel_burn_button {
                        let button = ImageButton::new(view.resources.texture_image("cancel"));
                        if ui.add(button).on_hover_text("Cancel current burn").clicked() {
                            events.push(Event::CancelCurrentSegment { entity });
                        }
                    }

                    if add_cancel_guidance_button {
                        let button = ImageButton::new(view.resources.texture_image("cancel"));
                        if ui.add(button).on_hover_text("Cancel current guidance").clicked() {
                            if model.vessel_component(entity).timeline().last_event().unwrap().is_intercept() {
                                // also cancel intercept
                                events.push(Event::CancelLastTimelineEvent { entity });
                            }
                            events.push(Event::CancelLastTimelineEvent { entity });
                        }
                    }
                });
            }
                
            Grid::new("Vessel bars grid").show(ui, |ui| {
                if !vessel_component.slots().fuel_tanks().is_empty() {
                    draw_fuel(ui, vessel_component);
                    draw_dv(ui, vessel_component);
                }
                draw_torpedoes(ui, vessel_component);
            });
        });
}
