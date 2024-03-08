use crate::{state::State, storage::entity_allocator::Entity};

use self::{ellipse_ellipse_bounder::get_bound, window::Window};

mod ellipse_ellipse_bounder;
mod sdf;
mod util;
mod window;

/// Finds bounds for all siblings of an entity
pub fn get_initial_windows(state: &State, entity: Entity, siblings: Vec<Entity>, start_time: f64, end_time: f64) -> Vec<Window> {
    let orbit = state.get_trajectory_component(entity).get_end_segment().as_orbit();
    let mut windows = vec![];

    for sibling in siblings {
        let other_orbit = state.get_trajectory_component(sibling).get_segment_at_time(start_time).as_orbit();
        // TODO this assumes it's ellipse-ellipse
        windows.append(&mut get_bound(orbit, other_orbit, sibling, start_time, end_time));
    }

    for window in &mut windows {
        if window.is_periodic() {
            while window.get_soonest_time() < start_time && window.get_latest_time() < start_time {
                *window = window.next()
            }
        }
    }

    // Remove non-periodic windows that don't have an intersection with the start_time to end_time window
    windows.retain(|window| {
        window.is_periodic() || !window.is_periodic() && window.get_latest_time() > start_time && window.get_soonest_time() < end_time
    });

    windows.sort_unstable_by(|a, b| a.cmp(b));

    windows
}