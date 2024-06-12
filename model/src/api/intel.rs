use std::collections::VecDeque;

use crate::{components::{orbitable_component::OrbitableComponentPhysics, path_component::{orbit::Orbit, segment::Segment}}, storage::entity_allocator::Entity, Model, SEGMENTS_TO_PREDICT};

impl Model {
    pub(crate) fn compute_perceived_path(&self, entity: Entity) -> VecDeque<Segment> {
        let current_segment = self.path_component(entity).current_segment();
        let parent = current_segment.parent();
        let parent_mass = self.mass(parent);
        let mass = current_segment.current_mass();
        let position = current_segment.current_position();
        let velocity = current_segment.current_velocity();
        let orbit = Orbit::new(parent, mass, parent_mass, position, velocity, self.time);
        let mut segments = VecDeque::new();
        segments.push_back(Segment::Orbit(orbit));

        while segments.len() < SEGMENTS_TO_PREDICT + 1 {
            let last_orbit = segments.back_mut().unwrap().as_orbit_mut().unwrap();
            let Some(orbit) = self.next_orbit(entity, last_orbit) else {
                break;
            };
            segments.push_back(Segment::Orbit(orbit));
        }
        
        segments
    }

    /// Can be called for any entity
    /// Will return path component data for factions that the player has intel on
    /// ... but perceived path component data for other factions
    pub fn perceived_future_segments(&self, entity: Entity) -> VecDeque<Segment> {
        if self.vessel_component(entity).faction().player_has_intel() {
            self.path_component(entity).future_segments().clone() // TODO get rid of this clone and implement proper caching
        } else {
            self.compute_perceived_path(entity)
        }
    }

    pub fn perceived_future_orbits(&self, entity: Entity) -> Vec<Orbit> {
        if self.vessel_component(entity).faction().player_has_intel() {
            self.path_component(entity).future_orbits().iter().cloned().cloned().collect() // TODO get rid of this clone and implement proper caching
        } else {
            self.compute_perceived_path(entity).iter().map(|segment| segment.as_orbit().unwrap().clone()).collect()
        }
    }

    /// Safe to call for orbitables
    pub fn perceived_future_segment_at_time(&self, entity: Entity, time: f64) -> Segment {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            if let OrbitableComponentPhysics::Orbit(orbit) = orbitable_component.physics() {
                Segment::Orbit(orbit.clone()) // TODO remove clone
            } else {
                panic!("Attempt to get segment of stationary orbitable")
            }
        } else if self.vessel_component(entity).faction().player_has_intel() {
            self.path_component(entity).future_segment_at_time(time).clone() // TODO get rid of this clone and implement proper caching
        } else {
            // The reason we clamp to the end time is because encounter prediction is nondeterministic
            // So if we call encounter prediction one frame, get the time, and store the result, the
            // same call the next frame might be very slightly sooner. Then suppose we feed the result into
            // this function... oh, look, we got a panic because the stored time is slightly after the 
            // end segment. Yes this is a stupid solution but I don't know how to better solve it
            let perceived_path = self.compute_perceived_path(entity);
            let time = f64::min(time, perceived_path.back().unwrap().end_time());
            for segment in perceived_path {
                if time >= segment.start_time() && time <= segment.end_time() {
                    return segment
                }
            }
            panic!("No segment exists at the given time")
        }
    }

    /// Safe to call for orbitables
    pub fn perceived_future_orbit_at_time(&self, entity: Entity, time: f64) -> Orbit {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(_) => panic!("Attempt to get orbit of stationary orbitable"),
                OrbitableComponentPhysics::Orbit(orbit) => orbit.clone(), // TODO REMOVE CLONE
            }
        }
        self.perceived_future_segment_at_time(entity, time).as_orbit().expect("Perceived segment at time is not orbit").clone() // TODO remove clone
    }

    pub fn perceived_parent_at_time(&self, entity: Entity, time: f64) -> Option<Entity> {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.orbit().map(Orbit::parent);
        }

        if let Some(vessel_component) = self.try_vessel_component(entity) {
            if vessel_component.faction().player_has_intel() {
                return Some(self.path_component(entity).future_segment_at_time(time).parent());
            } else {
                return Some(self.perceived_future_orbit_at_time(entity, time).parent());
            }
        }

        None
    }
}