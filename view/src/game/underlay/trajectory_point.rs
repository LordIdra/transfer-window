use eframe::egui::{Context, Pos2, Rgba};
use log::trace;

use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::{util::add_textured_square, Scene};

const SELECT_DISTANCE: f64 = 32.0;
const SELECT_RADIUS: f64 = 4.0;
const HOVER_COLOR: Rgba = Rgba::from_rgba_premultiplied(0.7, 0.7, 0.7, 0.7);
const SELECTED_COLOR: Rgba = Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 1.0);

pub enum SelectedPoint {
    None,
    Hover((Entity, f64)),
    Selected((Entity, f64)),
}

impl SelectedPoint {
    fn is_hover(&self) -> bool {
        matches!(self, Self::Hover((_, _)))
    }

    fn is_selected(&self) -> bool {
        matches!(self, Self::Selected((_, _)))
    }
}

fn remove_if_outdated(view: &mut Scene, model: &Model) {
    if let SelectedPoint::Selected((_, time)) = view.selected_point {
        if time < model.get_time() {
            trace!("Selected point invalidated at time={time}");
            view.selected_point = SelectedPoint::None;
        }
    }
}

fn update_selected(view: &mut Scene, model: &Model, context: &Context, latest_window: Pos2, primary_clicked: bool) {
    let select_distance = SELECT_DISTANCE / view.camera.get_zoom();
    let latest_world = view.camera.window_space_to_world_space(model, latest_window, context.screen_rect());

    let Some((entity, time)) = model.get_closest_point_on_trajectory(latest_world, select_distance) else {
        if primary_clicked || view.selected_point.is_hover() {
            view.selected_point = SelectedPoint::None;
        }
        return 
    };
    
    if primary_clicked {
        trace!("Selected new point at time={time}");
        view.selected_point = SelectedPoint::Selected((entity, time));
        return;
    }

    if !view.selected_point.is_selected() {
        view.selected_point = SelectedPoint::Hover((entity, time));
    }
}

fn draw_selected(view: &mut Scene, model: &Model) {
    let select_radius = SELECT_RADIUS / view.camera.get_zoom();
    match view.selected_point {
        SelectedPoint::None => (),
        SelectedPoint::Hover((entity, time)) => {
            let mut vertices = vec![];
            let trajectory_component = model.get_trajectory_component(entity);
            let segment = trajectory_component.get_segment_at_time(time);
            let point = model.get_absolute_position(segment.get_parent()) + segment.get_position_at_time(time);
            add_textured_square(&mut vertices, point, select_radius, HOVER_COLOR);
            view.texture_renderers.get("circle").unwrap().lock().unwrap().add_vertices(&mut vertices);
        }
        SelectedPoint::Selected((entity, time)) => {
            let mut vertices = vec![];
            let trajectory_component = model.get_trajectory_component(entity);
            let segment = trajectory_component.get_segment_at_time(time);
            let point = model.get_absolute_position(segment.get_parent()) + segment.get_position_at_time(time);
            add_textured_square(&mut vertices, point, select_radius, SELECTED_COLOR);
            view.texture_renderers.get("circle").unwrap().lock().unwrap().add_vertices(&mut vertices);
        }
    }
}

pub fn update(view: &mut Scene, model: &Model, context: &Context) {
    remove_if_outdated(view, model);

    context.input(|input| {
        if let Some(latest_window) = input.pointer.latest_pos() {
            if !context.is_pointer_over_area() {
                update_selected(view, model, context, latest_window, input.pointer.primary_clicked());
            }
        }
    });

    draw_selected(view, model);
}