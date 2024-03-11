use std::collections::HashSet;

use crate::{components::ComponentType, constants::MIN_TIME_BEFORE_ENCOUNTER, state::State, storage::entity_allocator::Entity, systems::trajectory_prediction::encounter::{Encounter, EncounterType}};

use self::{entrance_solver::solve_for_entrance, exit_solver::solve_for_exit};

use super::bounding::get_initial_windows;

mod entrance_solver;
mod exit_solver;

/// Returns all entities with the same FINAL parent from can_enter
/// It's expected that candidates only contains entities with a trajectory component
pub fn get_final_siblings(state: &State, candidates: &HashSet<Entity>, entity: Entity) -> Vec<Entity> {
    let end_segment = state.get_trajectory_component(entity).get_end_segment();
    let time = end_segment.get_end_time();
    let parent = end_segment.get_parent();
    let mut siblings = vec![];
    for other_entity in candidates {
        if entity == *other_entity {
            continue;
        }
        let other_end_segment = state.get_trajectory_component(*other_entity).get_segment_at_time(time);
        if parent != other_end_segment.get_parent() {
            continue;
        }
        siblings.push(*other_entity);
    }
    siblings
}

/// Solves for the next encounter of a single entity by combining the entrance and exit solvers
/// Problem:
/// - We have a bunch of windows between various times
/// - We need to find the soonest encounter
/// - We need to do it as quickly as possible
/// - We can assume the chance of an encounter occuring at any point in aany window is constant
/// - We might have a window or multiple that span the entire duration of start time to end time so just doing the earliest window isn't an option
/// - Caching is a thing so swapping between windows *may or may not* be expensive
/// - We have to choose the window to evaluate with the highest chance per unit time of giving us the soonest encounter
/// So this is basically a scheduling problem
/// Solution
/// - Define a maximum time for each window
/// - Iterate through each window on initialisation
///   - If the window is above maximum time, split it up at max time
/// - Now iterate windows, pick the soonest one, and evaluate for encounter
/// - Make sure that when evaluating a window, the window's end time does not exceed the global end time, and same for start times
/// - Once an encounter is evaluated, if it is periodic and the new window's soonest time exceeds end time, yeet the new window
/// - Keep going until either an encounter is found or the vec of windows is empty
/// - If an encounter is found, set the new end time to the time of the encounter and continue running the algorithm
pub fn find_next_encounter(state: &State, entity: Entity, start_time: f64, end_time: f64) -> Option<Encounter> {
    let can_enter = &state.get_entities(vec![ComponentType::TrajectoryComponent, ComponentType::OrbitableComponent]);
    let siblings = get_final_siblings(state, can_enter, entity);
    let mut windows = get_initial_windows(state, entity, siblings, start_time, end_time);
    let mut soonest_encounter = solve_for_exit(state, entity, start_time, end_time);
    let mut end_time = match &soonest_encounter {
        Some(encounter) => encounter.get_time(),
        None => end_time,
    };

    loop {
        windows.retain(|window| window.get_soonest_time() < end_time);
        windows.sort_by(|a, b| a.cmp(b));

        if windows.is_empty() {
            break;
        }

        let window = windows.pop().unwrap();
        let from = f64::max(window.get_soonest_time(), start_time);
        let to = f64::min(window.get_latest_time(), end_time);
        if let Some(encounter_time) = solve_for_entrance(window.get_orbit(), window.get_other_orbit(), from, to) {
            // Add a minimum time as another encounter could be calculated as being eg 0.01 seconds later
            // if eg an entity exits an SOI and then an 'entrance' is calculated to be very shortly after
            if encounter_time > start_time + MIN_TIME_BEFORE_ENCOUNTER {
                soonest_encounter = Some(Encounter::new(EncounterType::Entrance, entity, window.get_other_entity(), encounter_time));
                end_time = encounter_time;
            }
        }

        if window.is_periodic() {
            windows.push(window.next())
        }
    }

    soonest_encounter
}