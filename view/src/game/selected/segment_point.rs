use eframe::egui::PointerState;
use transfer_window_model::{components::vessel_component::faction::Faction, storage::entity_allocator::Entity};

use crate::game::{selected::Selected, util::add_textured_square, View};

pub const SELECT_DISTANCE: f64 = 24.0;
const SELECT_RADIUS: f64 = 4.0;
const HOVERED_ALPHA: f32 = 0.8;
const SELECTED_ALPHA: f32 = 1.0;

fn draw_selected_circle(view: &View, entity: Entity, time: f64, alpha: f32) {
    let select_radius = SELECT_RADIUS / view.camera.zoom();
    let mut vertices = vec![];
    let segment = view.model.segment_at_time(entity, time, Some(Faction::Player));
    let point = view.model.absolute_position(segment.parent()) + segment.position_at_time(time);
    add_textured_square(&mut vertices, point, select_radius, alpha);
    view.renderers.add_texture_vertices("circle", &mut vertices);
}

#[allow(clippy::too_many_arguments)]
pub fn draw_selected(view: &View) {
    match view.selected.clone() {
        Selected::BurnPoint { entity, time }
            | Selected::GuidancePoint { entity, time }
            | Selected::OrbitPoint { entity, time } => draw_selected_circle(view, entity, time, SELECTED_ALPHA),
        _ => ()
    }
}

pub fn draw_hover(view: &View, pointer: &PointerState) {
    if view.pointer_over_ui || view.pointer_over_icon {
        return;
    }

    let Some(latest_window) = pointer.latest_pos() else { 
        return;
    };
    
    let select_distance = SELECT_DISTANCE / view.camera.zoom();
    let latest_world = view.window_space_to_world_space(latest_window);
    if let Some((entity, time)) = view.model.closest_burn_point(latest_world, select_distance, Some(Faction::Player)) {
        draw_selected_circle(view, entity, time, HOVERED_ALPHA);
        return;
    }
    if let Some((entity, time)) = view.model.closest_guidance_point(latest_world, select_distance, Some(Faction::Player)) {
        draw_selected_circle(view, entity, time, HOVERED_ALPHA);
        return;
    }
    if let Some((entity, time)) = view.model.closest_orbit_point(latest_world, select_distance, Some(Faction::Player)) {
        draw_selected_circle(view, entity, time, HOVERED_ALPHA);
    }
}