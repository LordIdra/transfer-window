use log::trace;

use crate::{components::path_component::{orbit::Orbit, segment::Segment}, storage::entity_allocator::Entity, Model, SEGMENTS_TO_PREDICT};

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
    /// # Panics
    /// Panics if the last segment of the entity is a burn
    fn predict(&mut self, entity: Entity, end_time: f64, segment_count: usize) {
        if segment_count == 0 {
            return;
        }
        
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
                .expect("Attempt to predict when the last segment is a burn!")
                .end_at(end_time);
        }
    }

    pub fn recompute_trajectory(&mut self, entity: Entity) {
        // Add 1 because the final orbit will have duration 0
        let segments_to_predict = SEGMENTS_TO_PREDICT + 1 - self.path_component(entity).future_orbits_after_final_burn().len();
        dbg!(segments_to_predict);
        self.predict(entity, 1.0e10, segments_to_predict);
    }

    pub fn recompute_entire_trajectory(&mut self, entity: Entity) {
        let current_segment = self.path_component(entity).current_segment();
        let parent = current_segment.parent();
        let position = current_segment.current_position();
        let velocity = current_segment.current_velocity();
        let parent_mass = self.mass(parent);
        let mass = self.vessel_component(entity).mass();
        let time = self.time;
        let orbit = Orbit::new(parent, mass, parent_mass, position, velocity, time);

        self.path_component_mut(entity).clear_future_segments();
        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }
}
