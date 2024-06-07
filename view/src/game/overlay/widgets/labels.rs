use eframe::egui::{RichText, Ui};
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::util::{format_distance, format_speed, format_time};

pub fn draw_title(ui: &mut Ui, name: &str) {
    ui.label(RichText::new(name).size(20.0).monospace().strong());
}

pub fn draw_altitude(model: &Model, ui: &mut Ui, entity: Entity, time: f64) {
    ui.label(RichText::new("Altitude").monospace().strong());
    ui.label(format_distance(model.position_at_time(entity, time).magnitude()));
    ui.end_row();
}

pub fn draw_speed(model: &Model, ui: &mut Ui, entity: Entity, time: f64) {
    ui.label(RichText::new("Speed").monospace().strong());
    ui.label(format_speed(model.velocity_at_time(entity, time).magnitude()));
    ui.end_row();
}

pub fn draw_distance(model: &Model, ui: &mut Ui, entity: Entity, time: f64) {
    ui.label(RichText::new("Distance").monospace().strong());
    let distance = model.distance_at_time(entity, model.vessel_component(entity).target().unwrap(), time);
    ui.label(format_distance(distance));
    ui.end_row();
}

pub fn draw_orbits(model: &Model, ui: &mut Ui, entity: Entity, time: f64) {
    let orbit = model.orbit_at_time(entity, time);
    let Some(period) = orbit.period() else {
        return;
    };
    let orbits = ((time - orbit.current_point().time()) / period) as usize;
    if orbits != 0 {
        ui.label(RichText::new("Orbits:").strong().monospace());
        ui.label(orbits.to_string());
        ui.end_row();
    }
}

pub fn draw_time_until(model: &Model, ui: &mut Ui, time: f64) {
    let text = format!("T-{}", format_time(time - model.time()));
    ui.label(RichText::new(text).weak());
}

pub fn draw_encounter_to(model: &Model, ui: &mut Ui, entity: Entity) {
    ui.label(RichText::new("To").monospace().strong());
    ui.label(model.name_component(entity).name());
    ui.end_row();
}

pub fn draw_encounter_from(model: &Model, ui: &mut Ui, entity: Entity) {
    ui.label(RichText::new("From").monospace().strong());
    ui.label(model.name_component(entity).name());
    ui.end_row();
}