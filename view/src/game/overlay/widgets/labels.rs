use eframe::egui::{Color32, Grid, RichText, Ui};
use transfer_window_model::components::vessel_component::faction::Faction;
use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::util::{format_distance, format_speed, format_time};
use crate::game::View;

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

pub fn draw_value_with_color(ui: &mut Ui, text: &str, color: Color32) {
    ui.label(RichText::new(text).color(color).size(12.0));
}

pub fn draw_title(ui: &mut Ui, name: &str) {
    ui.label(RichText::new(name).size(20.0).monospace().strong());
}

pub fn draw_subtitle(ui: &mut Ui, name: &str) {
    ui.add_space(12.0);
    ui.label(RichText::new(name.to_uppercase()).size(18.0).monospace().strong());
}

pub fn draw_mass_at_time(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    ui.label(RichText::new("Mass").size(12.0).strong());
    ui.label(
        RichText::new(format!(
            "{} kg",
            view.model.mass_at_time(entity, time, Some(Faction::Player)).round()
        ))
        .size(12.0),
    );
    ui.end_row();
}

pub fn draw_altitude_at_time(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    draw_key(ui, "Altitude");
    draw_value(
        ui,
        &format_distance(
            view.model.position_at_time(entity, time, Some(Faction::Player)).magnitude(),
        ),
    );
    ui.end_row();
}

pub fn draw_speed_at_time(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    draw_key(ui, "Speed");
    draw_value(
        ui,
        &format_speed(view.model.velocity_at_time(entity, time, Some(Faction::Player)).magnitude()),
    );
    ui.end_row();
}

pub fn draw_mass(view: &View, ui: &mut Ui, entity: Entity) {
    ui.label(RichText::new("Mass").size(12.0).strong());
    ui.label(RichText::new(format!("{} kg", view.model.mass(entity).round())).size(12.0));
    ui.end_row();
}

pub fn draw_altitude(view: &View, ui: &mut Ui, entity: Entity) {
    ui.label(RichText::new("Altitude").size(12.0).strong());
    ui.label(RichText::new(format_distance(view.model.position(entity).magnitude())).size(12.0));
    ui.end_row();
}

pub fn draw_speed(view: &View, ui: &mut Ui, entity: Entity) {
    ui.label(RichText::new("Speed").size(12.0).strong());
    ui.label(RichText::new(format_speed(view.model.velocity(entity).magnitude())).size(12.0));
    ui.end_row();
}

pub fn draw_target_distance_at_time(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    let target = view.model.vessel_component(entity).target().unwrap();
    let distance = view.model.distance_at_time(entity, target, time, Some(Faction::Player));
    draw_key(ui, "Target distance");
    draw_value(ui, &format_distance(distance));
    ui.end_row();
}

pub fn draw_target_relative_speed_at_time(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    let target = view.model.vessel_component(entity).target().unwrap();
    let speed = view.model.relative_speed_at_time(entity, target, time, Some(Faction::Player));
    draw_key(ui, "Target relative speed");
    draw_value(ui, &format_speed(speed));
    ui.end_row();
}

pub fn draw_target_distance(view: &View, ui: &mut Ui, entity: Entity) {
    let target = view.model.vessel_component(entity).target().unwrap();
    let distance = view.model.distance(entity, target);
    draw_key(ui, "Target distance");
    draw_value(ui, &format_distance(distance));
    ui.end_row();
}

pub fn draw_target_relative_speed(view: &View, ui: &mut Ui, entity: Entity) {
    let target = view.model.vessel_component(entity).target().unwrap();
    let speed = view.model.relative_speed(entity, target);
    draw_key(ui, "Target relative speed");
    draw_value(ui, &format_speed(speed));
    ui.end_row();
}

pub fn draw_torpedo_launcher(view: &View, ui: &mut Ui, entity: Entity) {
    draw_key(ui, "Torpedo launcher");
    let cooldown = view.model.vessel_component(entity).torpedo_launcher_time_to_reload();
    if view.model.vessel_component(entity).torpedoes() == 0 {
        draw_value_with_color(ui, "Empty", Color32::from_rgb(255, 100, 100));
    } else if cooldown == 0.0 {
        draw_value_with_color(ui, "Ready", Color32::from_rgb(100, 255, 100));
    } else {
        draw_value(ui, &format_time(cooldown));
    }
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

pub fn draw_info(view: &View, ui: &mut Ui, name: &str, entity: Entity) {
    draw_subtitle(ui, "Info");
    Grid::new("Vessel info grid ".to_string() + name).show(ui, |ui| {
        draw_mass(view, ui, entity);
        draw_altitude(view, ui, entity);
        draw_speed(view, ui, entity);
        if view.model.vessel_component(entity).target().is_some() {
            draw_target_distance(view, ui, entity);
            draw_target_relative_speed(view, ui, entity);
        }
        if view.model.vessel_component(entity).has_torpedo_launcher() {
            draw_torpedo_launcher(view, ui, entity);
        }
    });
}

pub fn draw_info_at_time(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    draw_subtitle(ui, "Info");
    Grid::new("Selected approach info").show(ui, |ui| {
        draw_mass_at_time(view, ui, entity, time);
        draw_altitude_at_time(view, ui, entity, time);
        draw_speed_at_time(view, ui, entity, time);
        if view.model.vessel_component(entity).target().is_some() {
            draw_target_distance_at_time(view, ui, entity, time);
            draw_target_relative_speed_at_time(view, ui, entity, time);
        }
    });
}

pub fn draw_info_at_time_with_orbits(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    draw_subtitle(ui, "Info");
    Grid::new("Selected point info").show(ui, |ui| {
        draw_mass_at_time(view, ui, entity, time);
        draw_altitude_at_time(view, ui, entity, time);
        draw_speed_at_time(view, ui, entity, time);
        if view.model.vessel_component(entity).has_target() {
            draw_target_distance_at_time(view, ui, entity, time);
            draw_target_relative_speed_at_time(view, ui, entity, time);
        }
        draw_orbits(view, ui, entity, time);
    });
}
