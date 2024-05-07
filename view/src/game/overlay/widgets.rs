use eframe::{egui::{vec2, Color32, Pos2, Rect, Rounding, Sense, Stroke, Ui}, emath::RectTransform, epaint};

#[allow(clippy::too_many_arguments)]
pub fn draw_filled_bar(ui: &mut Ui, width: f32, height: f32, margin: f32, rounding: f32, filled_color: Color32, empty_color: Color32, proportion_filled: f32) {
    let (response, painter) = ui.allocate_painter(epaint::vec2(width, height), Sense::hover());
    let to_screen = RectTransform::from_to(
        Rect::from_min_size(Pos2::ZERO, response.rect.size()),
        response.rect,
    );

    let filled_width = (width) * proportion_filled;
    let from = to_screen.transform_pos(Pos2::new(margin, margin));
    let to = to_screen.transform_pos(Pos2::new(width - margin, height - margin));
    painter.rect(Rect::from_min_max(from, to), Rounding::same(rounding), empty_color, Stroke::NONE);

    let from = to_screen.transform_pos(Pos2::new(margin, margin));
    let to = to_screen.transform_pos(Pos2::new(filled_width - margin, height - margin));
    painter.rect(Rect::from_min_max(from, to), Rounding::same(rounding), filled_color, Stroke::NONE);
}

pub fn draw_scale_bar(ui: &mut Ui, width: f32, height: f32, line_width: f32, color: Color32, scale: f32) {
    let (response, painter) = ui.allocate_painter(epaint::vec2(width, height), Sense::hover());
    let to_screen = RectTransform::from_to(
        Rect::from_min_size(Pos2::ZERO, response.rect.size()),
        response.rect,
    );

    let center = Pos2::new(width / 2.0, height / 2.0);

    // Horizontal
    let from = to_screen.transform_pos(center + vec2(-scale / 2.0, 0.0));
    let to = to_screen.transform_pos(center + vec2(scale / 2.0, 0.0));
    painter.line_segment([from, to], Stroke::new(line_width, color));

    // Left vertical
    let from = to_screen.transform_pos(center + vec2(-scale / 2.0, -height / 3.0));
    let to = to_screen.transform_pos(center + vec2(-scale / 2.0, height / 3.0));
    painter.line_segment([from, to], Stroke::new(line_width, color));

    // Right vertical
    let from = to_screen.transform_pos(center + vec2(scale / 2.0, -height / 3.0));
    let to = to_screen.transform_pos(center + vec2(scale / 2.0, height / 3.0));
    painter.line_segment([from, to], Stroke::new(line_width, color));
}