use eframe::{egui::Ui, glow::{HasContext, RENDERER, SHADING_LANGUAGE_VERSION, VERSION}};

use crate::game::View;

pub fn draw(view: &View, ui: &mut Ui) {
    ui.label(format!("Operating System: {:?}", view.context.os()));
    ui.label(format!("Pixels/point: {:?}", view.context.pixels_per_point()));
    ui.label(format!("Native pixels/point: {:?}", view.context.native_pixels_per_point()));
    ui.label(format!("Frame: {}", view.context.frame_nr()));
    ui.label(format!("OpenGL version: {}", unsafe { view.gl.get_parameter_string(VERSION) }));
    ui.label(format!("OpenGL renderer: {}", unsafe { view.gl.get_parameter_string(RENDERER) }));
    ui.label(format!("GLSL version: {}", unsafe { view.gl.get_parameter_string(SHADING_LANGUAGE_VERSION) }));
}