use eframe::epaint::Rgba;
use nalgebra_glm::{vec2, DVec2, Vec2};

pub fn add_triangle(vertices: &mut Vec<f32>, v1: DVec2, v2: DVec2, v3: DVec2, color: Rgba) {
    let v1 = dvec2_to_f32_tuple(v1);
    let v2 = dvec2_to_f32_tuple(v2);
    let v3 = dvec2_to_f32_tuple(v3);

    vertices.push(v1.0.0);
    vertices.push(v1.0.1);
    vertices.push(v1.1.0);
    vertices.push(v1.1.1);
    vertices.push(color.r());
    vertices.push(color.g());
    vertices.push(color.b());
    vertices.push(color.a());

    vertices.push(v2.0.0);
    vertices.push(v2.0.1);
    vertices.push(v2.1.0);
    vertices.push(v2.1.1);
    vertices.push(color.r());
    vertices.push(color.g());
    vertices.push(color.b());
    vertices.push(color.a());

    vertices.push(v3.0.0);
    vertices.push(v3.0.1);
    vertices.push(v3.1.0);
    vertices.push(v3.1.1);
    vertices.push(color.r());
    vertices.push(color.g());
    vertices.push(color.b());
    vertices.push(color.a());

    // vertices.append(&mut vec![v1.0.0, v1.0.1, v1.1.0, v1.1.1, color.r(), color.g(), color.b(), color.a()]);
    // vertices.append(&mut vec![v2.0.0, v2.0.1, v2.1.0, v2.1.1, color.r(), color.g(), color.b(), color.a()]);
    // vertices.append(&mut vec![v3.0.0, v3.0.1, v3.1.0, v3.1.1, color.r(), color.g(), color.b(), color.a()]);
}

#[allow(clippy::too_many_arguments)]
pub fn add_textured_triangle(vertices: &mut Vec<f32>, v1: DVec2, v2: DVec2, v3: DVec2, color: Rgba, t1: Vec2, t2: Vec2, t3: Vec2) {
    let v1 = dvec2_to_f32_tuple(v1);
    let v2 = dvec2_to_f32_tuple(v2);
    let v3 = dvec2_to_f32_tuple(v3);
    vertices.append(&mut vec![v1.0.0, v1.0.1, v1.1.0, v1.1.1, color.r(), color.g(), color.b(), color.a(), t1.x, t1.y]);
    vertices.append(&mut vec![v2.0.0, v2.0.1, v2.1.0, v2.1.1, color.r(), color.g(), color.b(), color.a(), t2.x, t2.y]);
    vertices.append(&mut vec![v3.0.0, v3.0.1, v3.1.0, v3.1.1, color.r(), color.g(), color.b(), color.a(), t3.x, t3.y]);
}

pub fn add_textured_square(vertices: &mut Vec<f32>, position: DVec2, radius: f64, color: Rgba) {
    let v1 = vec2(position.x - radius, position.y - radius);
    let v2 = vec2(position.x - radius, position.y + radius);
    let v3 = vec2(position.x + radius, position.y - radius);
    let v4 = vec2(position.x + radius, position.y + radius);
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