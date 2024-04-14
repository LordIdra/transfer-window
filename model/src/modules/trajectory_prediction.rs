use log::trace;

use crate::{storage::entity_allocator::Entity, Model};

use self::fast_solver::{apply_encounter, solver::find_next_encounter};

mod encounter;
mod fast_solver;
#[cfg(test)]
mod test_cases;

/// Adds segments for all encounters after `entity`'s final
/// segment but before `end_time`. Obviously, `entity` is
/// expected to have a trajectory component. Additionally,
/// the final segment of entity's trajectory MUST be an Orbit.
/// Trajectory prediction is extremely complex, good luck
pub fn predict(model: &mut Model, entity: Entity, end_time: f64) {
    let mut start_time = model.get_trajectory_component(entity).get_end_segment().get_end_time();
    while let Some(encounter) = find_next_encounter(model, entity, start_time, end_time) {
        trace!("Found encounter {encounter:?}");
        apply_encounter(model, &encounter);
        start_time = encounter.get_time();
    }
    model.get_trajectory_component_mut(entity).get_end_segment_mut().as_orbit_mut().end_at(end_time);
}