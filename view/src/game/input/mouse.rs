use eframe::egui::{self, Context, Pos2, Rect};
use nalgebra_glm::vec2;

use super::Scene;

const ZOOM_SENSITIVITY: f64 = 0.003;

fn update_zoom(view: &mut Scene, latest_mouse_position: Pos2, scroll_delta: egui::Vec2, screen_size: Rect) {
    let screen_size = vec2(screen_size.width() as f64, screen_size.height() as f64);
    let new_zoom = view.camera.get_zoom() * (1.0 + ZOOM_SENSITIVITY * scroll_delta.y as f64);
    let delta_zoom = (view.camera.get_zoom() - new_zoom) / new_zoom;
    let mouse_position = vec2(
        -(latest_mouse_position.x as f64 - (screen_size.x / 2.0)), 
            latest_mouse_position.y as f64 - (screen_size.y / 2.0));
    view.camera.pan(mouse_position * delta_zoom);
    view.camera.set_zoom(new_zoom);
}



pub fn update(view: &mut Scene, context: &Context) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update mouse");
    context.input(|input| {
        if input.pointer.secondary_down() {
            let mouse_delta = input.pointer.delta();
            view.camera.pan(vec2(-mouse_delta.x as f64, mouse_delta.y as f64));
        };

        if let Some(latest_mouse_position) = input.pointer.latest_pos() {
            update_zoom(view, latest_mouse_position, input.scroll_delta, context.screen_rect());
        }
    });
}