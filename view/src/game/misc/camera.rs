use eframe::egui::{self, Key, Pos2, Rect, Vec2};
use nalgebra_glm::vec2;

use crate::game::events::ViewEvent;

use super::View;

pub const MIN_ZOOM: f64 = 1.0e-9;
pub const MAX_ZOOM: f64 = 100.0;
const ZOOM_SENSITIVITY: f64 = 0.003;

fn update_pan(view: &View, mouse_delta: Vec2) {
    let amount = vec2(-mouse_delta.x as f64, mouse_delta.y as f64);
    view.add_view_event(ViewEvent::PanCamera(amount));
}

fn update_zoom(view: &View, latest_mouse_position: Pos2, scroll_delta: egui::Vec2, screen_size: Rect) {
    let screen_size = vec2(screen_size.width() as f64, screen_size.height() as f64);
    let new_zoom = view.camera.zoom() * (1.0 + ZOOM_SENSITIVITY * scroll_delta.y as f64);
    let mouse_position = vec2(
        -(latest_mouse_position.x as f64 - (screen_size.x / 2.0)),
            latest_mouse_position.y as f64 - (screen_size.y / 2.0));

    let actual_new_zoom = new_zoom.clamp(MIN_ZOOM, MAX_ZOOM);
    let actual_delta_zoom = (view.camera.zoom() - actual_new_zoom) / actual_new_zoom;

    view.add_view_event(ViewEvent::PanCamera(mouse_position * actual_delta_zoom));
    view.add_view_event(ViewEvent::SetCameraZoom(actual_new_zoom));
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update camera");

    if view.pointer_over_ui {
        return;
    }

    let screen_rect = view.screen_rect;
    
    view.context.clone().input(|input| {
        if input.key_pressed(Key::R) {
            view.add_view_event(ViewEvent::ResetCameraPanning);
        }
        
        if input.key_pressed(Key::F) {
            if let Some(entity) = view.selected.entity(&view.model) {
                view.add_view_event(ViewEvent::SetCameraFocus(entity));
            }
        }
        
        if input.pointer.secondary_down() {
            update_pan(view, input.pointer.delta());
        };

        if let Some(latest_mouse_position) = input.pointer.latest_pos() {
            if input.smooth_scroll_delta.length() != 0.0 && !view.pointer_over_ui {
                update_zoom(view, latest_mouse_position, input.smooth_scroll_delta, screen_rect);
            }
        }
    });
}