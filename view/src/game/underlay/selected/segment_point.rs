use eframe::egui::{Context, PointerState, Rgba};
use log::trace;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::Selected, util::add_textured_square, Scene};

const SELECT_DISTANCE: f64 = 32.0;
const SELECT_RADIUS: f64 = 4.0;
const HOVER_COLOR: Rgba = Rgba::from_rgba_premultiplied(0.7, 0.7, 0.7, 0.7);
const SELECTED_COLOR: Rgba = Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 1.0);

#[derive(Debug, Clone)]
pub enum SegmentPointState {
    Hover,
    Selected,
}

impl SegmentPointState {
    pub fn is_hover(&self) -> bool {
        matches!(self, Self::Hover)
    }

    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected)
    }
}

pub fn remove_if_expired(view: &mut Scene, model: &Model, time: f64, state: &SegmentPointState) {
    if state.is_selected() && time < model.get_time() {
        trace!("Selected segment point expired at time={time}");
        view.selected = Selected::None;
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_selected(view: &mut Scene, model: &Model, context: &Context, pointer: &PointerState, is_mouse_over_ui_element: bool, entity: Entity, time: f64, state: &SegmentPointState) {
    // Deselected by clicking elsewhere
    if !is_mouse_over_ui_element && state.is_selected() && pointer.primary_clicked() {
        trace!("Selected segment point deselected");
        view.selected = Selected::None;
        return;
    }

    // Update if hovered
    if let Some(latest_window) = pointer.latest_pos() {
        if !is_mouse_over_ui_element && state.is_hover() {
            let select_distance = SELECT_DISTANCE / view.camera.get_zoom();
            let latest_world = view.camera.window_space_to_world_space(model, latest_window, context.screen_rect());
            view.selected = match model.get_closest_point_on_trajectory(latest_world, select_distance) {
                Some((entity, time)) => Selected::Point { entity, time, state: SegmentPointState::Hover },
                None => Selected::None,
            }
        }
    }

    // Select if hovered and clicked
    if state.is_hover() && pointer.primary_clicked() {
        trace!("Selected segment point at time={}", time);
        view.selected = Selected::Point { entity, time, state: SegmentPointState::Selected };
    }
}

pub fn update_not_selected(view: &mut Scene, model: &Model, context: &Context, pointer: &PointerState, is_mouse_over_ui_element: bool) {
    if !is_mouse_over_ui_element {
        if let Some(latest_window) = pointer.latest_pos() {
            let select_distance = SELECT_DISTANCE / view.camera.get_zoom();
            let latest_world = view.camera.window_space_to_world_space(model, latest_window, context.screen_rect());
            if let Some((entity, time)) = model.get_closest_point_on_trajectory(latest_world, select_distance) {
                view.selected = Selected::Point { entity, time, state: SegmentPointState::Hover };
            }
        }
    }
}

pub fn draw(view: &mut Scene, model: &Model) {
    let select_radius = SELECT_RADIUS / view.camera.get_zoom();
    if let Selected::Point { entity, time, state } = view.selected.clone() {
        let color = match state { 
            SegmentPointState::Hover => HOVER_COLOR,
            SegmentPointState::Selected => SELECTED_COLOR,
        };
        let mut vertices = vec![];
        let trajectory_component = model.get_trajectory_component(entity);
        let segment = trajectory_component.get_last_segment_at_time(time);
        let point = model.get_absolute_position(segment.get_parent()) + segment.get_position_at_time(time);
        add_textured_square(&mut vertices, point, select_radius, color);
        view.texture_renderers.get("circle").unwrap().lock().unwrap().add_vertices(&mut vertices);
    }
}