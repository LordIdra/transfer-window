use log::trace;

use crate::{storage::entity_allocator::Entity, Model};

use self::fast_solver::{apply_encounter, solver::find_next_encounter};

mod encounter;
mod fast_solver;
#[cfg(test)]
mod test_cases;

impl Model {
    /// Adds segments for all encounters after `entity`'s final
    /// segment but before `end_time`. Obviously, `entity` is
    /// expected to have a path component. Additionally,
    /// the final segment of entity's path MUST be an Orbit.
    /// Trajectory prediction is extremely complex, good luck if
    /// you need to modify this...
    pub fn predict(&mut self, entity: Entity, end_time: f64, segment_count: usize) {
        let mut start_time = self.path_component(entity).last_segment().end_time();
        let mut segments = 0;
        while let Some(encounter) = find_next_encounter(self, entity, start_time, end_time) {
            trace!("Found encounter {encounter:?}");
            apply_encounter(self, &encounter);
            start_time = encounter.time();
            segments += 1;
            if segments >= segment_count {
                break;
            }
        }
        
        if segments < segment_count {
            self.path_component_mut(entity)
                .last_segment_mut()
                .as_orbit_mut()
                .expect("Attempt to start prediction when the last segment is a burn!")
                .end_at(end_time);
        }
    }
}