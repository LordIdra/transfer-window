use eframe::egui::{ScrollArea, Ui};
use transfer_window_model::{components::{orbitable_component::{OrbitableComponent, OrbitableComponentPhysics}, path_component::{burn::{burn_point::BurnPoint, Burn}, guidance::{guidance_point::GuidancePoint, Guidance}, orbit::{orbit_point::OrbitPoint, Orbit}, segment::Segment, turn::{turn_point::TurnPoint, Turn}, PathComponent}}, model::Model, storage::entity_allocator::Entity};

use crate::game::util::format_time;

fn draw_burn_point(ui: &mut Ui, point: &BurnPoint) {
    ui.label(format!("Position: {:.3?}", point.position()));
    ui.label(format!("Velocity: {:.3?}", point.velocity()));
    ui.label(format!("Time: {:.3?}", point.time()));
}

fn draw_orbit_point(ui: &mut Ui, point: &OrbitPoint) {
    ui.label(format!("Position: {:.3?}", point.position()));
    ui.label(format!("Velocity: {:.3?}", point.velocity()));
    ui.label(format!("Time: {:.3?}", point.time()));
    ui.label(format!("Time since periapsis: {:.3?}", point.time_since_periapsis()));
    ui.label(format!("Theta: {:.3?}", point.theta()));
}

fn draw_guidance_point(ui: &mut Ui, point: &GuidancePoint) {
    ui.label(format!("Position: {:.3?}", point.position()));
    ui.label(format!("Velocity: {:.3?}", point.velocity()));
    ui.label(format!("PN Acceleration: {:.3?}", point.guidance_acceleration()));
    ui.label(format!("Time: {:.3?}", point.time()));
}

fn draw_turn_point(ui: &mut Ui, point: &TurnPoint) {
    ui.label(format!("Position: {:.3?}", point.position()));
    ui.label(format!("Velocity: {:.3?}", point.velocity()));
    ui.label(format!("Rotation: {:.3?}", point.rotation()));
    ui.label(format!("Mass: {:.3?}", point.mass()));
    ui.label(format!("Time: {:.3?}", point.time()));
}

fn draw_orbit(ui: &mut Ui, orbit: &Orbit) {
    ui.label(format!("Duration: {}", format_time(orbit.end_point().time() - orbit.start_point().time())));
    ui.label(format!("Remaining orbits: {}", orbit.remaining_orbits()));
    ui.label(format!("Direction: {:?}", orbit.direction()));
    match orbit.period() {
        Some(period) => {
            ui.label("Type: ellipse".to_string());
            ui.label(format!("Period: {}", format_time(period)));
        }
        None => {
            ui.label("Type: hyperbola".to_string());
        }
    }
    ui.label(format!("Semi-major axis: {:.5e}", orbit.semi_major_axis()));
    ui.label(format!("Semi-minor axis: {:.5e}", orbit.semi_minor_axis()));
    ui.label(format!("Eccentricity: {:.5}", orbit.eccentricity()));
    ui.label(format!("Argument of periapsis: {:.5e}", orbit.argument_of_periapsis()));
    ui.label(format!("Remaining angle: {:.5e}", orbit.remaining_angle()));
    ui.collapsing("Start", |ui| draw_orbit_point(ui, orbit.start_point()));
    ui.collapsing("Current", |ui| draw_orbit_point(ui, orbit.current_point()));
    ui.collapsing("End", |ui| draw_orbit_point(ui, orbit.end_point()));
}

fn draw_burn(ui: &mut Ui, burn: &Burn) {
    ui.label(format!("DV: {:.3?}", burn.total_dv()));
    ui.label(format!("Duration: {}", format_time(burn.duration())));
    ui.collapsing("Start", |ui| draw_burn_point(ui, burn.start_point()));
    ui.collapsing("Current", |ui| draw_burn_point(ui, burn.current_point()));
    ui.collapsing("End", |ui| draw_burn_point(ui, burn.end_point()));
}

fn draw_orbitable(ui: &mut Ui, orbitable_component: &OrbitableComponent) {
    ui.label(format!("Mass: {:.3e}", orbitable_component.mass()));
    ui.label(format!("Radius: {:.3e}", orbitable_component.radius()));
    ui.label(format!("Type: {:?}", orbitable_component.type_()));
    match orbitable_component.physics() {
        OrbitableComponentPhysics::Stationary(position) => { ui.label(format!("Position: {position:.3?}")); }
        OrbitableComponentPhysics::Orbit(segment) => { draw_orbit(ui, segment.as_orbit().unwrap()); }
    }
}

fn draw_guidance(ui: &mut Ui, guidance: &Guidance) {
    ui.label(format!("Duration: {}", format_time(guidance.duration())));
    ui.collapsing("Start", |ui| draw_guidance_point(ui, guidance.start_point()));
    ui.collapsing("Current", |ui| draw_guidance_point(ui, guidance.current_point()));
    ui.collapsing("End", |ui| draw_guidance_point(ui, guidance.end_point()));
}

fn draw_turn(ui: &mut Ui, turn: &Turn) {
    ui.label(format!("Duration: {}", format_time(turn.duration())));
    ui.collapsing("Start", |ui| draw_turn_point(ui, turn.start_point()));
    ui.collapsing("Current", |ui| draw_turn_point(ui, turn.current_point()));
    ui.collapsing("End", |ui| draw_turn_point(ui, turn.end_point()));
}

fn draw_path(ui: &mut Ui, path_component: &PathComponent) {
    for segment in path_component.future_segments() {
        match segment {
            Segment::Orbit(orbit) => draw_orbit(ui, orbit),
            Segment::Burn(burn) => draw_burn(ui, burn),
            Segment::Guidance(guidance) => draw_guidance(ui, guidance),
            Segment::Turn(turn) => draw_turn(ui, turn),
        }
    } 
}

fn draw_entity(model: &Model, ui: &mut Ui, entity: Entity) {
    if let Some(orbitable_component) = model.try_orbitable_component(entity) {
        ui.collapsing("Orbitable", |ui| {
            draw_orbitable(ui, orbitable_component);
        });
    }

    if let Some(path_component) = model.try_path_component(entity) {
        ui.collapsing("Path", |ui| {
            draw_path(ui, path_component);
        });
    }
}

fn draw_row(model: &Model, ui: &mut Ui, entity: Entity) {
    let name = match model.try_name_component(entity) {
        Some(name_component) => name_component.name(),
        None => "<unnamed>".to_string(),
    };
    ui.collapsing(name, |ui| draw_entity(model, ui, entity));
}

pub fn draw(model: &Model, ui: &mut Ui) {
    let entities: Vec<Entity> = model.entities(vec![]).into_iter().collect();
    ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, 10.0, entities.len(), |ui, row_range| {
        for i in row_range {
            draw_row(model, ui, entities[i]);
        }
    });
}
