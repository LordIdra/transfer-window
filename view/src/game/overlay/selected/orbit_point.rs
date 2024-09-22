use eframe::{egui::{Align2, Grid, Ui, Window}, epaint};
use transfer_window_model::story_event::StoryEvent;
use transfer_window_model::{components::{path_component::orbit::Orbit, vessel_component::faction::Faction}, storage::entity_allocator::Entity};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{buttons::{draw_create_burn, draw_create_turn, draw_enable_guidance, draw_fire_torpedo, draw_next, draw_previous, draw_select_vessel, draw_warp_to}, labels::{draw_info_at_time_with_orbits, draw_key, draw_subtitle, draw_time_until, draw_title, draw_value}}, selected::{util::BurnState, Selected}, util::{format_distance, format_time}, View}, styles};

use super::vessel::visual_timeline::draw_visual_timeline;

fn draw_controls(view: &View, entity: Entity, ui: &mut Ui, time: f64) {
    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);

        if draw_select_vessel(view, ui, entity) {
            view.add_view_event(ViewEvent::SetSelected(Selected::Vessel(entity)));
        }

        if let Some(time) = draw_previous(view, ui, time, entity) {
            let selected = Selected::OrbitPoint { entity, time };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }

        if let Some(time) = draw_next(view, ui, time, entity) {
            let selected = Selected::OrbitPoint { entity, time };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }

        if draw_warp_to(view, ui, time) {
            view.add_model_event(ModelEvent::StartWarp { end_time: time });
        }

        
        let faction = view.model.vessel_component(entity).faction();
        if Faction::Player.can_control(faction) {
            if draw_create_burn(view, ui, entity, time) {
                let selected = Selected::Burn { entity, time, state: BurnState::Selected };
                view.add_model_event(ModelEvent::CreateBurn { entity, time });
                view.add_view_event(ViewEvent::SetSelected(selected));
                view.add_story_event(StoryEvent::CreateBurn(entity));
            }

            if draw_create_turn(view, ui, entity, time) {
                let selected = Selected::Turn { entity, time };
                view.add_model_event(ModelEvent::CreateTurn { entity, time });
                view.add_view_event(ViewEvent::SetSelected(selected));
                view.add_story_event(StoryEvent::CreateTurn(entity));
            }

            if draw_enable_guidance(view, ui, entity, time) {
                let selected = Selected::EnableGuidance { entity, time };
                view.add_model_event(ModelEvent::CreateGuidance { entity, time });
                view.add_view_event(ViewEvent::SetSelected(selected));
                view.add_story_event(StoryEvent::EnableGuidance(entity));
            }

            if draw_fire_torpedo(view, ui, entity, time) {
                let selected = Selected::FireTorpedo { entity, time, state: BurnState::Selected };
                view.add_model_event(ModelEvent::CreateFireTorpedo { entity, time });
                view.add_view_event(ViewEvent::SetSelected(selected));
                view.add_story_event(StoryEvent::FireTorpedo(entity));
            }
        }

    });
}

pub fn draw_orbit_labels(view: &View, ui: &mut Ui, orbit: &Orbit) {
    if orbit.is_ellipse() {
        draw_key(ui, "Orbiting");
        draw_value(ui, &view.model.name_component(orbit.parent()).name());
        ui.end_row();

        draw_key(ui, "Direction");
        draw_value(ui, &format!("{:?}", orbit.direction()));
        ui.end_row();

        draw_key(ui, "Period");
        draw_value(ui, &format_time(orbit.period().unwrap()));
        ui.end_row();

        draw_key(ui, "Semi-major axis");
        draw_value(ui, &format_distance(orbit.semi_major_axis()));
        ui.end_row();

        draw_key(ui, "Eccentricity");
        draw_value(ui, &format!("{:.3}", orbit.eccentricity()));
        ui.end_row();
    } else {
        draw_key(ui, "Orbiting");
        draw_value(ui, &view.model.name_component(orbit.parent()).name());
        ui.end_row();

        draw_key(ui, "Direction");
        draw_value(ui, &format!("{:?}", orbit.direction()));
        ui.end_row();

        draw_key(ui, "Eccentricity");
        draw_value(ui, &format!("{:.3}", orbit.eccentricity()));
        ui.end_row();
    }
}

fn draw_orbit(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    let orbit = view.model.orbit_at_time(entity, time, Some(Faction::Player));
    draw_subtitle(ui, "Orbit");
    Grid::new("Selected point orbit info").show(ui, |ui| {
        draw_orbit_labels(view, ui, orbit);
    });
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update orbit point");
    let Selected::OrbitPoint { entity, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected point")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, "Orbit");
        draw_time_until(view, ui, time);
        draw_controls(view, entity, ui, time);
        draw_info_at_time_with_orbits(view, ui, entity, time);
        draw_orbit(view, ui, entity, time);
        draw_visual_timeline(view, ui, entity, time, true);
    });
}
