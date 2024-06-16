use eframe::{egui::{Align2, Color32, Grid, RichText, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::{faction::Faction, VesselComponent}, storage::entity_allocator::Entity};
use visual_timeline::draw_visual_timeline;

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::{vessel_editor::VesselEditor, widgets::{bars::{draw_filled_bar, FilledBar}, buttons::{draw_cancel_burn, draw_cancel_guidance, draw_edit_vessel, draw_focus}, labels::{draw_key, draw_subtitle, draw_title, draw_value}}}, selected::Selected, util::{format_distance, format_speed}, View}, styles};

pub mod visual_timeline;

fn draw_altitude(view: &View, ui: &mut Ui, entity: Entity) {
    ui.label(RichText::new("Altitude").size(12.0).strong());
    ui.label(RichText::new(format_distance(view.model.position(entity).magnitude())).size(12.0));
    ui.end_row();
}

fn draw_speed(view: &View, ui: &mut Ui, entity: Entity) {
    ui.label(RichText::new("Speed").size(12.0).strong());
    ui.label(RichText::new(format_speed(view.model.velocity(entity).magnitude())).size(12.0));
    ui.end_row();
}

fn should_draw_fuel(vessel_component: &VesselComponent) -> bool {
    vessel_component.has_fuel_tank()
}

fn draw_fuel(ui: &mut Ui, vessel_component: &VesselComponent) {
    let remaining_fuel = vessel_component.fuel_litres();
    let max_fuel = vessel_component.max_fuel_litres();
    let fuel_proportion = (remaining_fuel / max_fuel) as f32;
    draw_key(ui, "Fuel");
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::DARK_GRAY, vec![FilledBar::new(Color32::WHITE, fuel_proportion)]);
    draw_value(ui, &format!("{} / {} L", remaining_fuel.round(), max_fuel));
    ui.end_row();
}

fn should_draw_dv(vessel_component: &VesselComponent) -> bool {
    vessel_component.dv().is_some() && vessel_component.max_dv().is_some()
}

fn draw_dv(ui: &mut Ui, vessel_component: &VesselComponent) {
    let remaining_dv = vessel_component.dv().unwrap();
    let max_dv = vessel_component.max_dv().unwrap();
    let dv_proportion = (remaining_dv / max_dv) as f32;
    draw_key(ui, "ΔV");
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::DARK_GRAY, vec![FilledBar::new(Color32::WHITE, dv_proportion)]);
    draw_value(ui, &format!("{} / {} m/s", remaining_dv.round(), max_dv.round()));
    ui.end_row();
}

fn should_draw_torpedoes(vessel_component: &VesselComponent) -> bool {
    vessel_component.as_ship().is_some_and(|ship| ship.max_torpedoes() != 0)
}

fn draw_torpedoes(ui: &mut Ui, vessel_component: &VesselComponent) {
    let ship = &vessel_component.as_ship().unwrap();
    let max_torpedoes = ship.max_torpedoes();
    let torpedoes = ship.torpedoes();

    let dv_proportion = torpedoes as f32 / max_torpedoes as f32;
    draw_key(ui, "Torpedoes");
    draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::DARK_GRAY, vec![FilledBar::new(Color32::WHITE, dv_proportion)]);
    draw_value(ui, &format!("{torpedoes} / {max_torpedoes}"));
    ui.end_row();
}

fn draw_resources(ui: &mut Ui, vessel_component: &VesselComponent, name: &str) {
    if should_draw_fuel(vessel_component) || should_draw_dv(vessel_component) || should_draw_torpedoes(vessel_component) {
        draw_subtitle(ui, "Resources");
        Grid::new("Vessel resource grid ".to_string() + name).show(ui, |ui| {
            if should_draw_fuel(vessel_component) {
                draw_fuel(ui, vessel_component);
            }
            if should_draw_dv(vessel_component) {
                draw_dv(ui, vessel_component);
            }
            if should_draw_torpedoes(vessel_component) {
                draw_torpedoes(ui, vessel_component);
            }
        });
    }
}

fn draw_controls(vessel_component: &VesselComponent, view: &View, entity: Entity, ui: &mut Ui, has_intel: bool) {
    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);

        if draw_focus(view, ui) {
            view.add_view_event(ViewEvent::ResetCameraPanning);
            view.add_view_event(ViewEvent::SetCameraFocus(entity));
        }

        if has_intel && vessel_component.can_edit_ever() && draw_edit_vessel(view, ui, entity) {
            let vessel_editor = Some(VesselEditor::new(entity));
            view.add_view_event(ViewEvent::SetVesselEditor(vessel_editor));
        }

        if has_intel && view.model.path_component(entity).current_segment().is_burn() && draw_cancel_burn(view, ui) {
            view.add_model_event(ModelEvent::CancelCurrentSegment { entity });
        }

        if has_intel && view.model.path_component(entity).current_segment().is_guidance() && draw_cancel_guidance(view, ui) {
            if view.model.vessel_component(entity).timeline().last_event().is_some_and(|event| event.is_intercept()) {
                // also cancel intercept
                view.add_model_event(ModelEvent::CancelLastTimelineEvent { entity });
            }
            view.add_model_event(ModelEvent::CancelCurrentSegment { entity });
        }
    });
}

fn draw_info(view: &View, ui: &mut Ui, name: &str, entity: Entity) {
    draw_subtitle(ui, "Info");
    Grid::new("Vessel info grid ".to_string() + name).show(ui, |ui| {
        draw_altitude(view, ui, entity);
        draw_speed(view, ui, entity);
    });
}


pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update vessel");
    let Selected::Vessel(entity) = view.selected.clone() else { 
        return
    };

    let name = view.model.name_component(entity).name();
    let vessel_component = view.model.vessel_component(entity);
    let has_intel = Faction::Player.has_intel_for(vessel_component.faction());
    let has_control = Faction::Player.can_control(vessel_component.faction());

    Window::new("Selected vessel ".to_string() + &name)
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, &name);
        draw_controls(vessel_component, view, entity, ui, has_control);
        draw_info(view, ui, &name, entity);

        if has_intel {
            draw_resources(ui, vessel_component, &name);
        }

        draw_visual_timeline(view, ui, entity, view.model.time(), false);
    });
}
