use eframe::egui::Rgba;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{orbitable_component::OrbitableComponentPhysics, path_component::segment::Segment, ComponentType}, storage::entity_allocator::Entity};

use crate::game::{util::{add_line, should_render_parent}, View};

mod burn;
mod guidance;
mod orbit;

/// Draws a line between two points so that all the lines on a segment are connected together
/// This should be called multiple times with different i's to create a blur effect, where i represents how far away from the 'real' line this line is
fn add_orbit_line(vertices: &mut Vec<f32>, previous_point: &DVec2, new_point: &DVec2, color: Rgba) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Add orbit line");

    add_line(vertices, *previous_point, *new_point, color);
}

fn draw_from_points(view: &mut View, points: &[DVec2], color: Rgba) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw from points");
    let mut vertices = vec![];
    let mut previous_point = None;
    for new_point in points {
        if let Some(previous_point) = previous_point {
            add_orbit_line(&mut vertices, previous_point, new_point, color);
        }
        previous_point = Some(new_point);
    }
    view.renderers.add_segment_vertices(&mut vertices);
}

fn draw_path_segments(view: &mut View, entity: Entity, camera_centre: DVec2) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw segments for one entity");
    let zoom = view.camera.zoom();
    let path_component = view.model.path_component(entity);

    let mut segment_points_data = vec![];
    for segment in path_component.future_segments() {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Draw segment");
        let absolute_parent_position = view.model.absolute_position(segment.parent());
        match segment {
            Segment::Orbit(orbit) => {
                // When predicting trajectories, the last orbit will have duration zero, so skip it
                if orbit.duration().abs() == 0.0 {
                    continue;
                }
                let points = orbit::compute_points(orbit, absolute_parent_position, camera_centre, zoom);
                let color = orbit::compute_color_vessel(view, entity);
                if should_render_parent(view, orbit.parent()) {
                    segment_points_data.push((points, color));
                }
            },

            Segment::Burn(burn) => {
                let points = burn::compute_points(burn, absolute_parent_position, camera_centre, zoom);
                let color = burn::compute_color();
                if should_render_parent(view, burn.parent()) {
                    segment_points_data.push((points, color));
                }
            }

            Segment::Guidance(guidance) => {
                let points = guidance::compute_points(guidance, absolute_parent_position, camera_centre, zoom);
                let color = guidance::compute_color();
                if should_render_parent(view, guidance.parent()) {
                    segment_points_data.push((points, color));
                }
            }
        };
    }

    // Reverse to make sure that the segments are rendered in order
    // of how soon they are, so that closer segments take priority
    // over further ones
    for (segment_points, color) in segment_points_data.iter().rev() {
        draw_from_points(view, segment_points, *color);
    }
}

fn draw_orbitable_segment(view: &mut View, entity: Entity, camera_centre: DVec2) {
    let orbitable_component = view.model.orbitable_component(entity);
    if let OrbitableComponentPhysics::Orbit(orbit) = orbitable_component.physics() {
        let absolute_parent_position = view.model.absolute_position(orbit.parent());
        let points = orbit::compute_points(orbit, absolute_parent_position, camera_centre, view.camera.zoom());
        let color = orbit::compute_color_orbitable(view, entity);
        draw_from_points(view, &points, color);
    }
}

pub fn draw(view: &mut View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw segments");
    let camera_centre = view.camera.translation(&view.model);
    for entity in view.model.entities(vec![ComponentType::PathComponent]) {
        draw_path_segments(view, entity, camera_centre);
    }
    for entity in view.entities_should_render(vec![ComponentType::OrbitableComponent]) {
        draw_orbitable_segment(view, entity, camera_centre);
    }
}