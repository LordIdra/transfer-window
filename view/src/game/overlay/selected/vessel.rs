use docking::draw_docking;
use eframe::egui::{Align2, Color32, Grid, Ui, Window};
use eframe::epaint;
use transfer_window_model::components::vessel_component::faction::Faction;
use transfer_window_model::components::vessel_component::VesselComponent;
use transfer_window_model::storage::entity_allocator::Entity;
use visual_timeline::draw_visual_timeline;

use crate::game::events::{ModelEvent, ViewEvent};
use crate::game::overlay::widgets::bars::{draw_filled_bar, FilledBar};
use crate::game::overlay::widgets::buttons::{
    draw_cancel_burn, draw_cancel_guidance, draw_dock, draw_focus,
};
use crate::game::overlay::widgets::labels::{
    draw_info, draw_key, draw_subtitle, draw_title, draw_value,
};
use crate::game::overlay::widgets::util::{
    should_draw_dv, should_draw_fuel, should_draw_torpedoes,
};
use crate::game::selected::Selected;
use crate::game::View;
use crate::styles;

mod docking;
pub mod visual_timeline;

pub fn draw_fuel(ui: &mut Ui, vessel_component: &VesselComponent, color: Color32) {
    let remaining_fuel = vessel_component.fuel_litres();
    let max_fuel = vessel_component.fuel_capacity_litres();
    let fuel_proportion = (remaining_fuel / max_fuel) as f32;

    draw_key(ui, "Fuel");
    draw_filled_bar(
        ui,
        120.0,
        10.0,
        2.0,
        3.0,
        Color32::GRAY,
        vec![FilledBar::new(color, fuel_proportion)],
    );
    draw_value(ui, &format!("{} / {} L", remaining_fuel.round(), max_fuel));
}

pub fn draw_dv(ui: &mut Ui, vessel_component: &VesselComponent, color: Color32) {
    let remaining_dv = vessel_component.dv();
    let max_dv = vessel_component.max_dv();
    let dv_proportion = (remaining_dv / max_dv) as f32;

    draw_key(ui, "ΔV");
    draw_filled_bar(
        ui,
        120.0,
        10.0,
        2.0,
        3.0,
        Color32::GRAY,
        vec![FilledBar::new(color, dv_proportion)],
    );
    draw_value(
        ui,
        &format!("{} / {} m/s", remaining_dv.round(), max_dv.round()),
    );
}

pub fn draw_torpedoes(ui: &mut Ui, vessel_component: &VesselComponent, color: Color32) {
    let max_torpedoes = vessel_component.torpedo_capacity();
    let torpedoes = vessel_component.torpedoes();
    let dv_proportion = torpedoes as f32 / max_torpedoes as f32;

    draw_key(ui, "Torpedoes");
    draw_filled_bar(
        ui,
        120.0,
        10.0,
        2.0,
        3.0,
        Color32::GRAY,
        vec![FilledBar::new(color, dv_proportion)],
    );
    draw_value(ui, &format!("{torpedoes} / {max_torpedoes}"));
}

fn draw_resources_grid(ui: &mut Ui, vessel_component: &VesselComponent, name: &str) {
    Grid::new("Vessel resource grid ".to_string() + name).show(ui, |ui| {
        if should_draw_dv(vessel_component) {
            draw_dv(ui, vessel_component, Color32::WHITE);
            ui.end_row();
        }
        if should_draw_fuel(vessel_component) {
            draw_fuel(ui, vessel_component, Color32::WHITE);
            ui.end_row();
        }
        if should_draw_torpedoes(vessel_component) {
            draw_torpedoes(ui, vessel_component, Color32::WHITE);
            ui.end_row();
        }
    });
}

fn draw_resources(ui: &mut Ui, vessel_component: &VesselComponent, name: &str) {
    if should_draw_fuel(vessel_component)
        || should_draw_dv(vessel_component)
        || should_draw_torpedoes(vessel_component)
    {
        draw_subtitle(ui, "Resources");
        draw_resources_grid(ui, vessel_component, name);
    }
}

fn draw_controls(
    vessel_component: &VesselComponent,
    view: &View,
    entity: Entity,
    ui: &mut Ui,
    has_intel: bool,
) {
    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);

        if draw_focus(view, ui) {
            view.add_view_event(ViewEvent::ResetCameraPanning);
            view.add_view_event(ViewEvent::SetCameraFocus(entity));
        }

        if has_intel
            && view.model.path_component(entity).current_segment().is_burn()
            && draw_cancel_burn(view, ui)
        {
            view.add_model_event(ModelEvent::CancelCurrentSegment { entity });
        }

        if has_intel
            && view.model.path_component(entity).current_segment().is_guidance()
            && draw_cancel_guidance(view, ui)
        {
            if view
                .model
                .vessel_component(entity)
                .timeline()
                .last_event()
                .is_some_and(|event| event.is_intercept())
            {
                // also cancel intercept
                view.add_model_event(ModelEvent::CancelLastTimelineEvent { entity });
            }
            view.add_model_event(ModelEvent::CancelCurrentSegment { entity });
        }

        if has_intel && vessel_component.timeline().last_blocking_event().is_none() {
            if let Some(target) = vessel_component.target() {
                if view.model.try_vessel_component(target).is_some()
                    && view.model.can_ever_dock_to_target(entity)
                    && draw_dock(view, ui, entity)
                {
                    view.add_model_event(ModelEvent::Dock {
                        station: target,
                        entity,
                    });
                    view.add_view_event(ViewEvent::SetCameraFocus(target));
                    view.add_view_event(ViewEvent::SetSelected(Selected::Vessel(target)));
                }
            }
        }
    });
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update vessel");
    let Selected::Vessel(entity) = view.selected.clone() else {
        return;
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
                if view.model.vessel_component(entity).has_docking() {
                    draw_docking(view, ui, entity);
                }
            }

            draw_visual_timeline(view, ui, entity, view.model.time(), false);
        });
}
