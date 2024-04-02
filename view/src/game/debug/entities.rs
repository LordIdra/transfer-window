use eframe::egui::{ScrollArea, Ui};
use transfer_window_model::{components::trajectory_component::{burn::{burn_point::BurnPoint, Burn}, orbit::{orbit_point::OrbitPoint, Orbit}, segment::Segment}, storage::entity_allocator::Entity, Model};

use crate::game::util::format_time;

fn draw_burn_point(ui: &mut Ui, burn_point: &BurnPoint) {
    ui.label(format!("Position: {:.3?}", burn_point.get_position()));
    ui.label(format!("Velocity: {:.3?}", burn_point.get_velocity()));
    ui.label(format!("Time: {:.3?}", burn_point.get_time()));
}

fn draw_orbit_point(ui: &mut Ui, orbit_point: &OrbitPoint) {
    ui.label(format!("Position: {:.3?}", orbit_point.get_position()));
    ui.label(format!("Velocity: {:.3?}", orbit_point.get_velocity()));
    ui.label(format!("Time: {:.3?}", orbit_point.get_time()));
    ui.label(format!("Time since periapsis: {:.3?}", orbit_point.get_time_since_periapsis()));
    ui.label(format!("Theta: {:.3?}", orbit_point.get_theta()));
}

fn draw_orbit(ui: &mut Ui, orbit: &Orbit) {
    ui.label(format!("Duration: {}", format_time(orbit.get_end_point().get_time() - orbit.get_start_point().get_time())));
    ui.label(format!("Remaining orbits: {}", orbit.get_remaining_orbits()));
    ui.label(format!("Direction: {:?}", orbit.get_direction()));
    match orbit.get_period() {
        Some(period) => {
            ui.label("Type: ellipse".to_string());
            ui.label(format!("Period: {}", format_time(period)));
        }
        None => {
            ui.label("Type: hyperbola".to_string());
        }
    }
    ui.label(format!("Semi-major axis: {:.5e}", orbit.get_semi_major_axis()));
    ui.label(format!("Semi-minor axis: {:.5e}", orbit.get_semi_minor_axis()));
    ui.label(format!("Eccentricity: {:.5}", orbit.get_eccentricity()));
    ui.label(format!("Argument of periapsis: {:.5e}", orbit.get_argument_of_periapsis()));
    ui.label(format!("Remaining angle: {:.5e}", orbit.get_remaining_angle()));
    ui.collapsing("Start", |ui| draw_orbit_point(ui, orbit.get_start_point()));
    ui.collapsing("Current", |ui| draw_orbit_point(ui, orbit.get_current_point()));
    ui.collapsing("End", |ui| draw_orbit_point(ui, orbit.get_end_point()));
}

fn draw_burn(ui: &mut Ui, burn: &Burn) {
    ui.label(format!("DV: {:.3?}", burn.get_total_dv()));
    ui.label(format!("Duration: {}", format_time(burn.get_duration())));
    ui.collapsing("Start", |ui| draw_burn_point(ui, burn.get_start_point()));
    ui.collapsing("Current", |ui| draw_burn_point(ui, burn.get_current_point()));
    ui.collapsing("End", |ui| draw_burn_point(ui, burn.get_end_point()));
}

fn draw_entity(model: &Model, ui: &mut Ui, entity: Entity) {
    if let Some(mass_component) = model.try_get_mass_component(entity) {
        ui.label(format!("Mass: {:.3}", mass_component.get_mass()));
    }

    let is_orbitable = model.try_get_orbitable_component(entity).is_some();
    ui.label(format!("Orbitable: {is_orbitable}"));

    if let Some(stationary_component) = model.try_get_stationary_component(entity) {
        ui.label(format!("Position: {:.3?}", stationary_component.get_position()));
    }

    if let Some(trajectory_component) = model.try_get_trajectory_component(entity) {
        ui.collapsing("Trajectory", |ui| {
            for segment in trajectory_component.get_segments().iter().flatten() {
                match segment {
                    Segment::Orbit(orbit) => draw_orbit(ui, orbit),
                    Segment::Burn(burn) => draw_burn(ui, burn),
                }
            } 
        });
    }
}

pub fn draw(model: &Model, ui: &mut Ui) {
    let entities: Vec<Entity> = model.get_entities(vec![]).into_iter().collect();
    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show_rows(ui, 10.0, entities.len(), |ui, row_range| {
            for i in row_range {
                let entity = entities[i];
                let name = match model.try_get_name_component(entity) {
                    Some(name_component) => name_component.get_name(),
                    None => "<unnamed>".to_string(),
                };
                ui.collapsing(name, |ui| draw_entity(model, ui, entity));
            }
    });
}