use std::collections::VecDeque;

use crate::{components::path_component::{orbit::Orbit, segment::Segment}, storage::entity_allocator::Entity, Model, SEGMENTS_TO_PREDICT};

impl Model {
    pub fn path_without_intel(&self, entity: Entity) -> VecDeque<Segment> {
        let current_segment = self.path_component(entity).current_segment();
        let parent = current_segment.parent();
        let parent_mass = self.mass(parent);
        let mass = current_segment.current_mass();
        let position = current_segment.current_position();
        let velocity = current_segment.current_velocity();
        let orbit = Orbit::new(parent, mass, parent_mass, position, velocity, self.time);
        let mut segments = VecDeque::new();
        segments.push_back(Segment::Orbit(orbit));

        while segments.len() < SEGMENTS_TO_PREDICT {
            let last_orbit = segments.back_mut().unwrap().as_orbit_mut().unwrap();
            let Some(orbit) = self.next_orbit(entity, &last_orbit) else {
                break;
            };
            last_orbit.end_at(orbit.start_point().time());
            segments.push_back(Segment::Orbit(orbit));
        }

        segments.back_mut().unwrap().as_orbit_mut().unwrap().end_at(1.0e10);
        segments
    }
}