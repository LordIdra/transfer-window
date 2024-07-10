use eframe::egui::Ui;
use transfer_window_model::components::vessel_component::VesselComponent;

pub fn advance_cursor_to(ui: &mut Ui, x: f32) {
    let width = x - ui.cursor().left();
    let mut rect = ui.cursor();
    rect.set_width(width);
    rect.set_height(0.0);
    ui.advance_cursor_after_rect(rect);
}

pub fn should_draw_fuel(vessel_component: &VesselComponent) -> bool {
    vessel_component.has_fuel_tank()
}

pub fn should_draw_dv(vessel_component: &VesselComponent) -> bool {
    vessel_component.has_engine() && vessel_component.has_fuel_tank()
}

pub fn should_draw_torpedoes(vessel_component: &VesselComponent) -> bool {
    vessel_component.has_torpedo_storage()
}
