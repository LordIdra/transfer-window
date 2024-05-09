use eframe::egui::{self, Context, Key, Pos2, Rect, Vec2};
use nalgebra_glm::vec2;
use transfer_window_model::Model;

use super::Scene;

pub const MIN_ZOOM: f64 = 5.0e-7;
pub const MAX_ZOOM: f64 = 1.0;
const ZOOM_SENSITIVITY: f64 = 0.003;

fn update_pan(view: &mut Scene, mouse_delta: Vec2) {
    view.camera.pan(vec2(-mouse_delta.x as f64, mouse_delta.y as f64));
}

fn update_zoom(view: &mut Scene, latest_mouse_position: Pos2, scroll_delta: egui::Vec2, screen_size: Rect) {
    let screen_size = vec2(screen_size.width() as f64, screen_size.height() as f64);
    let new_zoom = view.camera.zoom() * (1.0 + ZOOM_SENSITIVITY * scroll_delta.y as f64);
    let mouse_position = vec2(
        -(latest_mouse_position.x as f64 - (screen_size.x / 2.0)),
            latest_mouse_position.y as f64 - (screen_size.y / 2.0));

    let actual_new_zoom = f64::max(MIN_ZOOM, f64::min(MAX_ZOOM, new_zoom));
    let actual_delta_zoom = (view.camera.zoom() - actual_new_zoom) / actual_new_zoom;

    view.camera.pan(mouse_position * actual_delta_zoom);
    view.camera.set_zoom(actual_new_zoom);
}

pub fn update(view: &mut Scene, model: &Model, context: &Context) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update camera");
    
    context.input(|input| {
        if input.key_pressed(Key::R) {
            view.camera.reset_panning();
        }
        
        if input.key_pressed(Key::F) {
            if let Some(entity) = view.selected.entity(model) {
                view.camera.set_focus(Some(entity));
            }
        }
        
        if input.pointer.secondary_down() {
            update_pan(view, input.pointer.delta());
        };

        if let Some(latest_mouse_position) = input.pointer.latest_pos() {
            if input.scroll_delta.length() != 0.0 {
                update_zoom(view, latest_mouse_position, input.scroll_delta, context.screen_rect());
            }
        }
    });
}