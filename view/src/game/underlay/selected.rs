use eframe::egui::{Context, Pos2, Rgba};
use log::trace;

use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::{util::add_textured_square, Scene};

const SELECT_DISTANCE: f64 = 32.0;
const SELECT_RADIUS: f64 = 4.0;
const HOVER_COLOR: Rgba = Rgba::from_rgba_premultiplied(0.7, 0.7, 0.7, 0.7);
const SELECTED_COLOR: Rgba = Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 1.0);

#[derive(Debug, Clone)]
pub enum PointState {
    Hover,
    Selected,
}

impl PointState {
    pub fn is_hover(&self) -> bool {
        matches!(self, Self::Hover)
    }

    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected)
    }
}

#[derive(Debug, Clone)]
pub enum BurnState {
    Selected,
    Adjusting,
}

impl BurnState {
    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected)
    }

    pub fn is_adjusting(&self) -> bool {
        matches!(self, Self::Adjusting)
    }
}

#[derive(Debug, Clone)]
pub enum Selected {
    None,
    Point { entity: Entity, time: f64, state: PointState },
    Burn { entity: Entity, time: f64, state: BurnState }
}

fn update_none(view: &mut Scene, model: &Model, context: &Context, latest_window: Pos2) {
    let select_distance = SELECT_DISTANCE / view.camera.get_zoom();
    let latest_world = view.camera.window_space_to_world_space(model, latest_window, context.screen_rect());
    if let Some((entity, time)) = model.get_closest_point_on_trajectory(latest_world, select_distance) {
        view.selected = Selected::Point { entity, time, state: PointState::Hover };
    }
}

#[allow(clippy::too_many_arguments)]
fn update_point(view: &mut Scene, model: &Model, context: &Context, latest_window: Pos2, primary_clicked: bool, entity: Entity, time: f64, state: &PointState) {
    // Remove if expired
    if state.is_selected() && time < model.get_time() {
        trace!("Selected point expired at time={time}");
        view.selected = Selected::None;
        return;
    }

    // Deselected by clicking elsewhere
    if state.is_selected() && primary_clicked {
        trace!("Selected point deselected");
        view.selected = Selected::None;
        return;
    }

    // Update if hovered
    if state.is_hover() {
        let select_distance = SELECT_DISTANCE / view.camera.get_zoom();
        let latest_world = view.camera.window_space_to_world_space(model, latest_window, context.screen_rect());
        view.selected = match model.get_closest_point_on_trajectory(latest_world, select_distance) {
            Some((entity, time)) => Selected::Point { entity, time, state: PointState::Hover },
            None => Selected::None,
        }
    }

    // Select if hovered and clicked
    if state.is_hover() && primary_clicked {
        trace!("Selected point at time={}", time);
        view.selected = Selected::Point { entity, time, state: PointState::Selected };
    }
}

fn update_burn(view: &mut Scene, model: &Model, primary_clicked: bool, time: f64, state: &BurnState) {
    // Remove if expired
    if state.is_selected() && time < model.get_time() {
        trace!("Selected burn expired at time={time}");
        view.selected = Selected::None;
        return;
    }

    // Deselected by clicking elsewhere
    if (state.is_selected() || state.is_adjusting()) && primary_clicked {
        trace!("Selected burn deselected");
        view.selected = Selected::None;
    }
}

fn draw_selected(view: &mut Scene, model: &Model) {
    let select_radius = SELECT_RADIUS / view.camera.get_zoom();
    if let Selected::Point { entity, time, state } = view.selected.clone() {
        let color = match state { 
            PointState::Hover => HOVER_COLOR,
            PointState::Selected => SELECTED_COLOR,
        };
        let mut vertices = vec![];
        let trajectory_component = model.get_trajectory_component(entity);
        let segment = trajectory_component.get_segment_at_time(time);
        let point = model.get_absolute_position(segment.get_parent()) + segment.get_position_at_time(time);
        add_textured_square(&mut vertices, point, select_radius, color);
        view.texture_renderers.get("circle").unwrap().lock().unwrap().add_vertices(&mut vertices);
    }
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, is_mouse_over_any_icon: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    context.input(|input| {
        if let Some(latest_window) = input.pointer.latest_pos() {
            if !context.is_pointer_over_area() && !is_mouse_over_any_icon {
                match view.selected.clone() {
                    Selected::None => update_none(view, model, context, latest_window),
                    Selected::Point { entity, time, state } => update_point(view, model, context, latest_window, input.pointer.primary_clicked(), entity, time, &state),
                    Selected::Burn { entity: _, time, state } => update_burn(view, model, input.pointer.primary_clicked(), time, &state),
                }
            }
        }
    });

    draw_selected(view, model);
}