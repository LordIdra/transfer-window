use crate::{components::path_component::orbit::Orbit, model::Model, storage::entity_allocator::Entity};

use self::{ellipse::compute_ellipse_bound, hyperbola::compute_hyperbola_bound, window::Window};

mod ellipse;
mod hyperbola;
mod sdf;
mod util;
mod window;

/// Finds bounds for all siblings of an entity
pub fn compute_initial_windows<'a>(
    model: &'a Model, 
    orbit: &'a Orbit, 
    siblings: Vec<Entity>, 
    end_time: f64
) -> Result<Vec<Window<'a>>, &'static str> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Get initial windows");
    let start_time = orbit.start_point().time();
    let mut windows = vec![];

    for sibling in siblings {
        let sibling_orbit = model.orbitable_component(sibling).orbit().unwrap();
        assert!(sibling_orbit.is_ellipse(), "Orbitable is on hyperbolic trajectory");
        if orbit.is_ellipse() {
            for window in compute_ellipse_bound(orbit, sibling_orbit, sibling, start_time)? {
                windows.push(window);
            }
        } else {
            for window in compute_hyperbola_bound(orbit, sibling_orbit, sibling)? {
                windows.push(window);
            }
        }
    }

    // Increment each window until it doesn't occur in its entirety before the start time
    // Then make sure the start and end times of the window are clamped to the global start/end times
    for window in &mut windows {
        if window.is_periodic() {
            while window.soonest_time() < start_time && window.latest_time() < start_time {
                *window = window.next();
            }
        }
    }

    // Remove non-periodic windows that don't have an intersection with the start_time to end_time window
    windows.retain(|window| {
        window.is_periodic() || !window.is_periodic() && window.latest_time() > start_time && window.soonest_time() < end_time
    });

    Ok(windows)
}
