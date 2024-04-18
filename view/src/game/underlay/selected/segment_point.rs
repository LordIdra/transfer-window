use eframe::egui::{Context, PointerState};
use log::trace;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::Selected, util::add_textured_square, Scene};

const SELECT_DISTANCE: f64 = 24.0;
const SELECT_RADIUS: f64 = 4.0;
const HOVERED_ALPHA: f32 = 0.8;
const SELECTED_ALPHA: f32 = 1.0;

fn draw_selected_circle(view: &mut Scene, model: &Model, entity: Entity, time: f64, alpha: f32) {
    let select_radius = SELECT_RADIUS / view.camera.get_zoom();
    let mut vertices = vec![];
    let trajectory_component = model.get_trajectory_component(entity);
    let segment = trajectory_component.get_last_segment_at_time(time);
    let point = model.get_absolute_position(segment.get_parent()) + segment.get_position_at_time(time);
    add_textured_square(&mut vertices, point, select_radius, alpha);
    view.texture_renderers.get("circle").unwrap().lock().unwrap().add_vertices(&mut vertices);
}

#[allow(clippy::too_many_arguments)]
pub fn draw_selected(view: &mut Scene, model: &Model) {
    if let Selected::Point { entity, time } = view.selected.clone() {
        draw_selected_circle(view, model, entity, time, SELECTED_ALPHA);
    }
}

pub fn draw_hover(view: &mut Scene, model: &Model, context: &Context, pointer: &PointerState, is_mouse_over_ui_element: bool) {
    if is_mouse_over_ui_element {
        return;
    }

    let Some(latest_window) = pointer.latest_pos() else { 
        return 
    };
    
    let select_distance = SELECT_DISTANCE / view.camera.get_zoom();
    let latest_world = view.camera.window_space_to_world_space(model, latest_window, context.screen_rect());
    if let Some((entity, time)) = model.get_closest_point_on_trajectory(latest_world, select_distance) {
        if !is_mouse_over_ui_element && pointer.primary_clicked() {
            trace!("Selected segment point at time={}", time);
            view.selected = Selected::Point { entity, time };
        } else {
            draw_selected_circle(view, model, entity, time, HOVERED_ALPHA);
        }
    }
}