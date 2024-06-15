use eframe::egui::{RichText, Ui};
use transfer_window_model::{components::vessel_component::Faction, storage::entity_allocator::Entity};

use crate::game::{util::{format_distance, format_speed, format_time}, View};

pub fn draw_time_until(view: &View, ui: &mut Ui, time: f64) {
    let text = format!("T-{}", format_time(time - view.model.time()));
    ui.label(RichText::new(text).size(12.0).weak());
}

pub fn draw_key(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).size(12.0).strong());
}

pub fn draw_value(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).size(12.0));
}

pub fn draw_title(ui: &mut Ui, name: &str) {
    ui.label(RichText::new(name).size(20.0).monospace().strong());
}

pub fn draw_subtitle(ui: &mut Ui, name: &str) {
    ui.add_space(12.0);
    ui.label(RichText::new(name.to_uppercase()).size(14.0).monospace().strong());
}

pub fn draw_altitude(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    draw_key(ui, "Altitude");
    draw_value(ui, &format_distance(view.model.position_at_time(entity, time, Some(Faction::Player)).magnitude()));
    ui.end_row();
}

pub fn draw_speed(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    draw_key(ui, "Speed");
    draw_value(ui, &format_speed(view.model.velocity_at_time(entity, time, Some(Faction::Player)).magnitude()));
    ui.end_row();
}

pub fn draw_distance(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    let distance = view.model.distance_at_time(entity, view.model.vessel_component(entity).target().unwrap(), time, Some(Faction::Player));
    draw_key(ui, "Distance");
    draw_value(ui, &format_distance(distance));
    ui.end_row();
}

pub fn draw_orbits(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    let orbit = view.model.orbit_at_time(entity, time, Some(Faction::Player));
    let Some(period) = orbit.period() else {
        return;
    };
    let orbits = ((time - orbit.current_point().time()) / period) as usize;
    if orbits != 0 {
        draw_key(ui, "Orbits");
        draw_value(ui, &orbits.to_string());
        ui.end_row();
    }
}

pub fn draw_encounter_to(view: &View, ui: &mut Ui, entity: Entity) {
    draw_key(ui, "To");
    draw_value(ui, &view.model.name_component(entity).name());
    ui.end_row();
}

pub fn draw_encounter_from(view: &View, ui: &mut Ui, entity: Entity) {
    draw_key(ui, "From");
    draw_value(ui, &view.model.name_component(entity).name());
    ui.end_row();
}
