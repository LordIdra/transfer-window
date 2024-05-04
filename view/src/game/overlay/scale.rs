use eframe::{egui::{Align, Align2, Color32, Context, Layout, Pos2, Rect, Sense, Stroke, Window}, emath::RectTransform, epaint};

use crate::game::Scene;

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
            
            let (response, painter) = ui.allocate_painter(epaint::vec2(120.0, 20.0), Sense::hover());

            let to_screen = RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );

            // Horizontal
            let from = to_screen.transform_pos(Pos2::new(60.0 - width / 2.0, 10.0));
            let to = to_screen.transform_pos(Pos2::new(60.0 + width / 2.0, 10.0));
            painter.line_segment([from, to], Stroke::new(2.0, Color32::WHITE));

            // Left vertical
            let from = to_screen.transform_pos(Pos2::new(60.0 - width / 2.0, 5.0));
            let to = to_screen.transform_pos(Pos2::new(60.0 - width / 2.0, 15.0));
            painter.line_segment([from, to], Stroke::new(2.0, Color32::WHITE));

            // Right vertical
            let from = to_screen.transform_pos(Pos2::new(60.0 + width / 2.0, 5.0));
            let to = to_screen.transform_pos(Pos2::new(60.0 + width / 2.0, 15.0));
            painter.line_segment([from, to], Stroke::new(2.0, Color32::WHITE));
        });
}