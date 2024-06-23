use eframe::egui::Ui;

pub fn advance_cursor_to(ui: &mut Ui, x: f32) {
    let width = x - ui.cursor().left();
    let mut rect = ui.cursor();
    rect.set_width(width);
    rect.set_height(0.0);
    ui.advance_cursor_after_rect(rect);
}