use eframe::egui::{RichText, Ui};
use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::{util::{format_distance, format_speed, format_time}, View};

pub fn draw_title(ui: &mut Ui, name: &str) {
    ui.label(RichText::new(name).size(20.0).monospace().strong());
}

pub fn draw_subtitle(ui: &mut Ui, name: &str) {
    ui.add_space(8.0);
    ui.label(RichText::new(name.to_uppercase()).size(14.0).monospace().strong());
}

pub fn draw_altitude(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    ui.label(RichText::new("Altitude").monospace().strong());
    ui.label(format_distance(view.model.position_at_time(entity, time).magnitude()));
    ui.end_row();
}

pub fn draw_speed(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    ui.label(RichText::new("Speed").monospace().strong());
    ui.label(format_speed(view.model.velocity_at_time(entity, time).magnitude()));
    ui.end_row();
}

pub fn draw_distance(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    ui.label(RichText::new("Distance").monospace().strong());
    let distance = view.model.distance_at_time(entity, view.model.vessel_component(entity).target().unwrap(), time);
    ui.label(format_distance(distance));
    ui.end_row();
}

pub fn draw_orbits(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    let orbit = view.model.orbit_at_time(entity, time);
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

pub fn draw_time_until(view: &View, ui: &mut Ui, time: f64) {
    let text = format!("T-{}", format_time(time - view.model.time()));
    ui.label(RichText::new(text).weak());
}

pub fn draw_encounter_to(view: &View, ui: &mut Ui, entity: Entity) {
    ui.label(RichText::new("To").monospace().strong());
    ui.label(view.model.name_component(entity).name());
    ui.end_row();
}

pub fn draw_encounter_from(view: &View, ui: &mut Ui, entity: Entity) {
    ui.label(RichText::new("From").monospace().strong());
    ui.label(view.model.name_component(entity).name());
    ui.end_row();
}