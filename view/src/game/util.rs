use eframe::epaint::Rgba;
use nalgebra_glm::{vec2, DVec2, Vec2};
use thousands::Separable;
use transfer_window_model::{components::{orbitable_component::OrbitableType, vessel_component::{class::VesselClass, faction::Faction, VesselComponent}}, storage::entity_allocator::Entity};

use super::{selected::util::BurnAdjustDirection, View};

pub const BURN_OFFSET: f64 = 40.0;
const MIN_SOI_PIXELS: f64 = 30.0;

#[allow(dead_code)] // You never know when you might need this function
pub fn add_triangle(vertices: &mut Vec<f32>, v1: DVec2, v2: DVec2, v3: DVec2, color: Rgba) {
    let v1 = dvec2_to_f32_tuple(v1);
    let v2 = dvec2_to_f32_tuple(v2);
    let v3 = dvec2_to_f32_tuple(v3);

    vertices.append(&mut vec![v1.0.0, v1.0.1, v1.1.0, v1.1.1, color.r(), color.g(), color.b(), color.a()]);
    vertices.append(&mut vec![v2.0.0, v2.0.1, v2.1.0, v2.1.1, color.r(), color.g(), color.b(), color.a()]);
    vertices.append(&mut vec![v3.0.0, v3.0.1, v3.1.0, v3.1.1, color.r(), color.g(), color.b(), color.a()]);
}

pub fn add_line(vertices: &mut Vec<f32>, v1: DVec2, v2: DVec2, color: Rgba) {
    let v1 = dvec2_to_f32_tuple(v1);
    let v2 = dvec2_to_f32_tuple(v2);

    vertices.append(&mut vec![v1.0.0, v1.0.1, v1.1.0, v1.1.1, color.r(), color.g(), color.b(), color.a()]);
    vertices.append(&mut vec![v2.0.0, v2.0.1, v2.1.0, v2.1.1, color.r(), color.g(), color.b(), color.a()]);
}

#[allow(clippy::too_many_arguments)]
pub fn add_textured_triangle(vertices: &mut Vec<f32>, v1: DVec2, v2: DVec2, v3: DVec2, alpha: f32, t1: Vec2, t2: Vec2, t3: Vec2) {
    let v1 = dvec2_to_f32_tuple(v1);
    let v2 = dvec2_to_f32_tuple(v2);
    let v3 = dvec2_to_f32_tuple(v3);
    vertices.append(&mut vec![v1.0.0, v1.0.1, v1.1.0, v1.1.1, alpha, t1.x, t1.y]);
    vertices.append(&mut vec![v2.0.0, v2.0.1, v2.1.0, v2.1.1, alpha, t2.x, t2.y]);
    vertices.append(&mut vec![v3.0.0, v3.0.1, v3.1.0, v3.1.1, alpha, t3.x, t3.y]);
}

pub fn add_textured_square(vertices: &mut Vec<f32>, position: DVec2, radius: f64, alpha: f32) {
    let v1 = vec2(position.x - radius, position.y - radius);
    let v2 = vec2(position.x - radius, position.y + radius);
    let v3 = vec2(position.x + radius, position.y - radius);
    let v4 = vec2(position.x + radius, position.y + radius);
    add_textured_triangle(vertices, v1, v2, v3, alpha, vec2(0.0, 1.0), vec2(0.0, 0.0), vec2(1.0, 1.0));
    add_textured_triangle(vertices, v4, v2, v3, alpha, vec2(1.0, 0.0), vec2(0.0, 0.0), vec2(1.0, 1.0));
}

pub fn add_textured_square_facing(vertices: &mut Vec<f32>, position: DVec2, radius: f64, color: f32, facing_unit: DVec2) {
    let perpendicular_unit = vec2(-facing_unit.y, facing_unit.x);
    let v1 = position + (perpendicular_unit - facing_unit) * radius;
    let v2 = position + (perpendicular_unit + facing_unit) * radius;
    let v3 = position - (perpendicular_unit + facing_unit) * radius;
    let v4 = position - (perpendicular_unit - facing_unit) * radius;
    add_textured_triangle(vertices, v1, v2, v3, color, vec2(0.0, 1.0), vec2(0.0, 0.0), vec2(1.0, 1.0));
    add_textured_triangle(vertices, v4, v2, v3, color, vec2(1.0, 0.0), vec2(0.0, 0.0), vec2(1.0, 1.0));
}

pub fn add_textured_rectangle_facing(vertices: &mut Vec<f32>, position: DVec2, dimensions: DVec2, color: f32, facing_unit: DVec2) {
    let perpendicular_unit = vec2(-facing_unit.y, facing_unit.x);
    let v1 = position + (perpendicular_unit * dimensions.y - facing_unit * dimensions.x);
    let v2 = position + (perpendicular_unit * dimensions.y + facing_unit * dimensions.x);
    let v3 = position - (perpendicular_unit * dimensions.y + facing_unit * dimensions.x);
    let v4 = position - (perpendicular_unit * dimensions.y - facing_unit * dimensions.x);
    add_textured_triangle(vertices, v1, v2, v3, color, vec2(0.0, 1.0), vec2(0.0, 0.0), vec2(1.0, 1.0));
    add_textured_triangle(vertices, v4, v2, v3, color, vec2(1.0, 0.0), vec2(0.0, 0.0), vec2(1.0, 1.0));
}

fn dvec2_to_f32_tuple(vec: DVec2) -> ((f32, f32), (f32, f32)) {
    (f64_to_f32_pair(vec.x), f64_to_f32_pair(vec.y))
}

pub fn f64_to_f32_pair(v: f64) -> (f32, f32) {
    let upper = v as f32;
    let lower = (v - upper as f64) as f32;
    (upper, lower)
}

pub fn format_time(time: f64) -> String {
    let start_string = if time.is_sign_positive() { "".to_string() } else { "-".to_string() };
    let time = time.abs();
    let years_quotient = f64::floor(time / (360.0 * 24.0 * 60.0 * 60.0));
    let years_remainder = time % (360.0 * 24.0 * 60.0 * 60.0);
    let days_quotient = f64::floor(years_remainder / (24.0 * 60.0 * 60.0));
    let days_remainder = years_remainder % (24.0 * 60.0 * 60.0);
    let hours_quotient = f64::floor(days_remainder / (60.0 * 60.0));
    let hours_remainder = days_remainder % (60.0 * 60.0);
    let minutes_quotient = f64::floor(hours_remainder / 60.0);
    let seconds = f64::round(hours_remainder % 60.0);
    if years_quotient != 0.0 {
        start_string
            + years_quotient.to_string().as_str() + "y"
            + days_quotient.to_string().as_str() + "d"
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if days_quotient != 0.0 {
        start_string
            + days_quotient.to_string().as_str() + "d"
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if hours_quotient != 0.0 {
        start_string
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if minutes_quotient != 0.0 {
        start_string
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else {
        start_string
            + seconds.to_string().as_str() + "s"
    }
}

pub fn format_time_with_millis(time: f64) -> String {
    if time < 1.0 {
        format!("{time:.2}s")
    } else if time < 10.0 {
        format!("{time:.1}s")
    } else {
        format_time(time)
    }
}

pub fn format_distance(distance: f64) -> String {
    if distance < 1_000.0 {
        format!("{} m", distance.round())
    } else if distance < 10_000.0 {
        format!("{:.3} km", (distance / 1000.0))
    } else if distance < 100_000.0 {
        format!("{:.2} km", (distance / 1000.0))
    } else if distance < 1_000_000.0 {
        format!("{:.1} km", (distance / 1000.0))
    } else {
        format!("{} km", (distance / 1000.0).round().separate_with_commas())
    }
}

pub fn format_speed(speed: f64) -> String {
    if speed < 1_000.0 {
        format!("{} m/s", speed.round())
    } else if speed < 10_000.0 {
        format!("{:.3} km/s", (speed / 1000.0))
    } else if speed < 100_000.0 {
        format!("{:.2} km/s", (speed / 1000.0))
    } else if speed < 1_000_000.0 {
        format!("{:.1} km/s", (speed / 1000.0))
    } else {
        format!("{} km/s", (speed / 1000.0).round().separate_with_commas())
    }
}

pub fn compute_burn_arrow_position(view: &View, entity: Entity, time: f64, direction: BurnAdjustDirection) -> DVec2 {
    let burn = view.model.burn_starting_at_time(entity, time);
    let burn_position = view.model.absolute_position(burn.parent()) + burn.start_point().position();
    let burn_to_arrow_unit = burn.rotation_matrix() * direction.vector();
    burn_position + BURN_OFFSET * burn_to_arrow_unit / view.camera.zoom()
}

pub fn compute_adjust_fire_torpedo_arrow_position(view: &View, entity: Entity, time: f64, direction: BurnAdjustDirection) -> DVec2 {
    let event = view.model.fire_torpedo_event_at_time(entity, time).expect("No fire torpedo event found");
    let orbit = view.model.orbit_at_time(entity, time, None);
    let burn_position = view.model.absolute_position(orbit.parent()) + view.model.position_at_time(entity, time, None);
    let burn_to_arrow_unit = view.model.burn_starting_at_time(event.ghost(), event.burn_time()).rotation_matrix() * direction.vector();
    burn_position + BURN_OFFSET * burn_to_arrow_unit / view.camera.zoom()
}

pub fn should_render_parent(view: &View, parent: Entity) -> bool {
    let Some(orbit) = view.model.orbitable_component(parent).orbit() else {
        return true;
    };
    let soi_pixels = orbit.sphere_of_influence() * view.camera.zoom();
    soi_pixels >= MIN_SOI_PIXELS
}

pub fn should_render(view: &View, entity: Entity) -> bool {
    let Some(parent) = view.model.parent(entity) else {
        return true;
    };
    should_render_parent(view, parent)
}

pub fn should_render_at_time(view: &View, entity: Entity, time: f64) -> bool {
    let Some(parent) = view.model.parent_at_time(entity, time, Some(Faction::Player)) else {
        return true;
    };
    should_render_parent(view, parent)
}

pub fn vessel_texture(vessel_component: &VesselComponent) -> &'static str {
    match vessel_component.class() {
        VesselClass::Scout1 => "vessel-icon-scout-1",
        VesselClass::Frigate1 => "vessel-icon-frigate-1",
        VesselClass::Torpedo => "vessel-icon-torpedo",
        VesselClass::Station => "vessel-icon-hub",
    }
}

pub fn orbitable_texture(type_: OrbitableType) -> &'static str {
    match type_ {
        OrbitableType::Star => "star",
        OrbitableType::Planet => "planet",
        OrbitableType::Moon => "moon",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApsisType {
    Periapsis,
    Apoapsis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApproachType {
    First,
    Second,
}
