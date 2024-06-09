use eframe::{egui::{Align2, Grid, Ui, Window}, epaint};
use transfer_window_model::{components::path_component::orbit::Orbit, storage::entity_allocator::Entity};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{buttons::{draw_create_burn, draw_enable_guidance, draw_next, draw_previous, draw_warp_to}, labels::{draw_altitude, draw_key, draw_orbits, draw_speed, draw_subtitle, draw_time_until, draw_title, draw_value}}, selected::{util::BurnState, Selected}, util::{format_distance, format_time}, View}, styles};

use self::weapons::draw_weapons;

use super::vessel::visual_timeline;

mod weapons;

fn draw_vessel(view: &View, entity: Entity, ui: &mut Ui, time: f64) {
    draw_title(ui, "Orbit");
    draw_time_until(view, ui, time);

    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);

        if let Some(time) = draw_previous(view, ui, time, entity) {
            let selected = Selected::Point { entity, time };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }

        if let Some(time) = draw_next(view, ui, time, entity) {
            let selected = Selected::Point { entity, time };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }

        if draw_warp_to(view, ui, time) {
            view.add_model_event(ModelEvent::StartWarp { end_time: time });
        }

        if draw_create_burn(view, ui, entity, time) {
            view.add_model_event(ModelEvent::CreateBurn { entity, time });
            let selected = Selected::Burn { entity, time, state: BurnState::Selected };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }

        if draw_enable_guidance(view, ui, entity, time) {
            view.add_model_event(ModelEvent::CreateGuidance { entity, time });
            let selected = Selected::EnableGuidance { entity, time };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }
    });
    
    draw_subtitle(ui, "Info");
    Grid::new("Selected point info").show(ui, |ui| {
        draw_altitude(view, ui, entity, time);
        draw_speed(view, ui, entity, time);
        draw_orbits(view, ui, entity, time);
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
    let orbit = view.model.orbit_at_time(entity, time);
    draw_subtitle(ui, "Orbit");
    Grid::new("Selected point orbit info").show(ui, |ui| {
        draw_orbit_labels(view, ui, orbit);
    });
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update point");
    let Selected::Point { entity, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected point")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_vessel(view, entity, ui, time);
        draw_orbit(view, ui, entity, time);
        visual_timeline::draw(view, ui, entity, time, true);
    });

    if !view.model.vessel_component(entity).slots().weapon_slots().is_empty() {
        Window::new("Weapons")
                .title_bar(false)
                .resizable(false)
                .anchor(Align2::CENTER_BOTTOM, epaint::vec2(0.0, 0.0))
                .show(&view.context.clone(), |ui| {
            let weapon_slots = view.model.vessel_component(entity).slots().weapon_slots();
            draw_weapons(view, ui, entity, &weapon_slots, time);
        });
    }
}
