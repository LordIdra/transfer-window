use std::collections::HashSet;

use crate::{components::ComponentType, api::trajectories::encounter::{Encounter, EncounterType}, storage::entity_allocator::Entity, Model};

use self::{entrance_solver::solve_for_entrance, exit_solver::solve_for_exit};

use super::bounding::compute_initial_windows;

mod entrance_solver;
mod exit_solver;

const MIN_TIME_BEFORE_ENCOUNTER: f64 = 1.0;

/// Returns all entities with the same FINAL parent from `can_enter`
/// It's expected that candidates only contains entities with an orbitable component,
/// and that entity has a path component
fn compute_siblings(model: &Model, candidates: &HashSet<Entity>, entity: Entity) -> Vec<Entity> {
    let end_segment = model.path_component(entity).last_segment();
    let parent = end_segment.parent();
    let mut siblings = vec![];
    for other_entity in candidates {
        if entity == *other_entity {
            continue;
        }
        if let Some(other_orbit) = model.orbitable_component(*other_entity).orbit() {
            if parent == other_orbit.parent() {
                siblings.push(*other_entity);
            }
        }
    }
    siblings
}

/// Solves for the next encounter of a single entity by combining the entrance and exit solvers
// - We create an ordered list of all the windows
// - We continually evaluate the soonest window for encounters
// - Once a window is evaluated, if it is periodic it will be incremented by a period, otherwise removed
// - If an encounter is found, bring the end time forward to the time of the encounter (so we don't continue to uselessly compute encounters after the soonest known encounter)
pub fn find_next_encounter(model: &Model, entity: Entity, start_time: f64, end_time: f64) -> Option<Encounter> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Find next encounter");

    // Find entrance windows
    let can_enter = &model.entities(vec![ComponentType::OrbitableComponent]);
    let siblings = compute_siblings(model, can_enter, entity);
    let mut windows = compute_initial_windows(model, entity, siblings, start_time, end_time);

    // If an exit encounter is found, set that as the soonest known encounter
    let mut soonest_encounter = solve_for_exit(model, entity, start_time, end_time);
    let mut end_time = match &soonest_encounter {
        Some(encounter) => encounter.time(),
        None => end_time,
    };

    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Solving windows");
    // Evaluate entrance windows from soonest to latest until an encounter is found or no windows remain
    // Once an encounter is found, we'll have to evaluate all the remaining windows that are not entirely after the time of the discovered encounter
    // Because an encounter with another object could possibly happen sooner than the encounter we just found
    loop {
        windows.retain(|window| window.soonest_time() < end_time);
        #[allow(clippy::redundant_closure_for_method_calls)] // We would have to make Window public to fix this
        windows.sort_by(|a, b| a.cmp(b));

        if windows.is_empty() {
            break;
        }

        // Popped window has the soonest start time
        let window = windows.pop().unwrap();
        let from = f64::max(window.soonest_time(), start_time);
        let to = f64::min(window.latest_time(), end_time);
        if let Some(encounter_time) = solve_for_entrance(window.orbit(), window.other_orbit(), from, to) {
            // Add a minimum time as another encounter could be calculated as being eg 0.01 seconds later
            // if eg an entity exits an SOI and then an 'entrance' is calculated to be very shortly after
            if encounter_time > start_time + MIN_TIME_BEFORE_ENCOUNTER {
                soonest_encounter = Some(Encounter::new(EncounterType::Entrance, entity, window.other_entity(), encounter_time));
                end_time = encounter_time;
            }
        }

        if window.is_periodic() {
            windows.push(window.next());
        }
    }

    soonest_encounter
}