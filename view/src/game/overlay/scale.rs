use eframe::{egui::{Align, Align2, Color32, Context, Layout, Window}, epaint};

use crate::game::Scene;

use super::widgets::bars::draw_scale_bar;

fn calculate_scale(pixels_per_metre: f32) -> (f32, String) {
    let mut width = pixels_per_metre;
    let mut scale = 1.0;
    while width < 10.0 {
        width *= 10.0;
        scale *= 10.0;
    }

    let mut suffix = "m";
    if scale >= 1.0e3 {
        scale /= 1.0e3;
        suffix = "km";
    }
    if scale >= 1.0e3 {
        scale /= 1.0e3;
        suffix = "Mm";
    }
    if scale >= 1.0e3 {
        scale /= 1.0e3;
        suffix = "Gm";
    }
    if scale >= 1.0e3 {
        scale /= 1.0e3;
        suffix = "Tm";
    }
    if scale >= 1.0e3 {
        scale /= 1.0e3;
        suffix = "Pm";
    }

    (width, scale.to_string() + suffix)
}

pub fn update(view: &Scene, context: &Context) {
    Window::new("Scale")
        .title_bar(false)
        .resizable(false)
        .default_width(120.0)
        .anchor(Align2::RIGHT_BOTTOM, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            let (width, scale) = calculate_scale(view.camera.zoom() as f32);
            ui.with_layout(Layout::default().with_cross_align(Align::Center), |ui| {
                ui.label(scale);
            });
            draw_scale_bar(ui, 120.0, 20.0, 2.0, Color32::WHITE, width);
        });
}