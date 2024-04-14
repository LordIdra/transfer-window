use eframe::egui::{Context, PointerState, Pos2, Rect, Rgba};
use log::trace;

use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::{util::add_textured_square, Scene}};

const SELECT_DISTANCE: f64 = 32.0;
const SELECT_RADIUS: f64 = 4.0;
const HOVER_COLOR: Rgba = Rgba::from_rgba_premultiplied(0.7, 0.7, 0.7, 0.7);
const SELECTED_COLOR: Rgba = Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 1.0);

#[derive(Debug, Clone, PartialEq)]
pub enum BurnAdjustDirection {
    Prograde,
    Retrograde,
    Normal,
    Antinormal,
}

impl BurnAdjustDirection {
    pub fn get_vector(&self) -> DVec2 {
        match self {
            BurnAdjustDirection::Prograde => vec2(1.0, 0.0),
            BurnAdjustDirection::Retrograde => vec2(-1.0, 0.0),
            BurnAdjustDirection::Normal => vec2(0.0, 1.0),
            BurnAdjustDirection::Antinormal => vec2(0.0, -1.0),
        }
    }
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum BurnState {
    Selected,
    Adjusting,
    Dragging(BurnAdjustDirection)
}

impl BurnState {
    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected)
    }

    pub fn is_adjusting(&self) -> bool {
        matches!(self, Self::Adjusting)
    }

    pub fn is_dragging(&self) -> bool {
        matches!(self, Self::Dragging(_))
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

fn burn_adjustment_amount(amount: f64) -> f64 {
    if amount.is_sign_positive() {
        (8.0e-3 * amount).powi(2)
    } else {
        4.0 * amount
    }
}

fn update_burn(view: &mut Scene, model: &Model, events: &mut Vec<Event>, pointer: &PointerState, screen_rect: Rect, time: f64) {
    // Remove if expired
    if time < model.get_time() {
        trace!("Selected burn expired at time={time}");
        view.selected = Selected::None;
        return;
    }

    // Deselected by clicking elsewhere
    if pointer.primary_clicked() {
        trace!("Selected burn deselected");
        view.selected = Selected::None;
    }
    
    // Finished dragging
    if let Selected::Burn { entity: _, time: _, state } = &mut view.selected {
        if state.is_dragging() {
            // We don't check primary_released() because this does not trigger if mouse is dragged outside of window
            if !pointer.primary_down() {
                trace!("Stopped dragging to adjust burn");
                *state = BurnState::Adjusting;
            }
        }
    }

    // Do drag adjustment
    if let Selected::Burn { entity, time, state: BurnState::Dragging(direction) } = view.selected.clone() {
        if let Some(mouse_position) = pointer.latest_pos() {
            let burn = model.get_trajectory_component(entity).get_last_segment_at_time(time).as_burn();
            let burn_position = model.get_absolute_position(burn.get_parent()) + burn.get_start_point().get_position();
            let burn_to_mouse = view.camera.window_space_to_world_space(model, mouse_position, screen_rect) - burn_position;
            let burn_to_arrow = burn.get_rotation_matrix() * direction.get_vector();
            let amount = burn_adjustment_amount(burn_to_mouse.dot(&burn_to_arrow)) * direction.get_vector() * view.camera.get_zoom().powi(2);
            events.push(Event::AdjustBurn { entity, time, amount });
        }
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
        let segment = trajectory_component.get_last_segment_at_time(time);
        let point = model.get_absolute_position(segment.get_parent()) + segment.get_position_at_time(time);
        add_textured_square(&mut vertices, point, select_radius, color);
        view.texture_renderers.get("circle").unwrap().lock().unwrap().add_vertices(&mut vertices);
    }
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, is_mouse_over_any_icon: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    context.input(|input| {
        if let Some(latest_window) = input.pointer.latest_pos() {
            if !context.is_pointer_over_area() && !is_mouse_over_any_icon {
                match view.selected.clone() {
                    Selected::None => update_none(view, model, context, latest_window),
                    Selected::Point { entity, time, state } => update_point(view, model, context, latest_window, input.pointer.primary_clicked(), entity, time, &state),
                    Selected::Burn { entity: _, time, state: _ } => update_burn(view, model, events, &input.pointer, context.screen_rect(), time),
                }
            }
        }
    });

    draw_selected(view, model);
}