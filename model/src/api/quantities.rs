use log::error;
use nalgebra_glm::{vec2, DVec2};

use crate::{components::{orbitable_component::OrbitableComponentPhysics, path_component::segment::Segment}, storage::entity_allocator::Entity, Model};

impl Model {
    /// # Panics
    /// Panics if entity does not have a position
    pub fn get_position(&self, entity: Entity) -> DVec2 {
        if let Some(orbitable_component) = self.try_get_orbitable_component(entity) {
            return orbitable_component.get_position();
        }

        if let Some(path_component) = self.try_get_path_component(entity) {
            return path_component.get_current_segment().current_position();
        }

        error!("Request to get position of entity without path or orbitable components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn get_position_at_time(&self, entity: Entity, time: f64) -> DVec2 {
        if let Some(orbitable_component) = self.try_get_orbitable_component(entity) {
            return orbitable_component.get_position();
        }

        if let Some(path_component) = self.try_get_path_component(entity) {
            return path_component.get_first_segment_at_time(time).position_at_time(time);
        }

        error!("Request to get position of entity without path or orbitable components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn get_absolute_position(&self, entity: Entity) -> DVec2 {
        if let Some(orbitable_component) = self.try_get_orbitable_component(entity) {
            return match orbitable_component.get_physics() {
                OrbitableComponentPhysics::Stationary(position) => *position,
                OrbitableComponentPhysics::Orbit(orbit) => self.get_absolute_position(orbit.parent()) + orbit.current_point().position(),
            }
        }

        if let Some(path_component) = self.try_get_path_component(entity) {
            let current_segment = path_component.get_current_segment();
            return self.get_absolute_position(current_segment.parent()) + current_segment.current_position();
        }

        error!("Request to get absolute position of entity without path or orbitable components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn get_velocity(&self, entity: Entity) -> DVec2 {
        if let Some(orbitable_component) = self.try_get_orbitable_component(entity) {
            return match orbitable_component.get_physics() {
                OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
                OrbitableComponentPhysics::Orbit(orbit) => orbit.current_point().velocity(),
            }
        }

        if let Some(path_component) = self.try_get_path_component(entity) {
            return path_component.get_current_segment().current_velocity();
        }

        error!("Request to get velocity of entity without path or orbitable components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn get_velocity_at_time(&self, entity: Entity, time: f64) -> DVec2 {
        if let Some(orbitable_component) = self.try_get_orbitable_component(entity) {
            return match orbitable_component.get_physics() {
                OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
                OrbitableComponentPhysics::Orbit(orbit) => orbit.point_at_time(time).velocity(),
            }
        }

        if let Some(path_component) = self.try_get_path_component(entity) {
            let segment_at_time = path_component.get_first_segment_at_time(time);
            return segment_at_time.velocity_at_time(time);
        }

        error!("Request to get velocity of entity without path or orbitable components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    /// # Panics
    /// Panics if entity does not have a velocity
    pub fn get_absolute_velocity(&self, entity: Entity) -> DVec2 {
        if let Some(orbitable_component) = self.try_get_orbitable_component(entity) {
            return match orbitable_component.get_physics() {
                OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
                OrbitableComponentPhysics::Orbit(orbit) => self.get_absolute_velocity(orbit.parent()) + orbit.current_point().velocity(),
            }
        }

        if let Some(path_component) = self.try_get_path_component(entity) {
            let current_segment = path_component.get_current_segment();
            return self.get_absolute_velocity(current_segment.parent()) + current_segment.current_velocity();
        }

        error!("Request to get absolute velocity of entity without path or orbitable components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    /// # Panics
    /// Panics if entity does not have a mass
    pub fn get_mass(&self, entity: Entity) -> f64 {
        if let Some(orbitable_component) = self.try_get_orbitable_component(entity) {
            return orbitable_component.get_mass();
        }

        if let Some(vessel_component) = self.try_get_vessel_component(entity) {
            if let Segment::Burn(burn) = self.get_path_component(entity).get_current_segment() {
                return burn.current_point().get_mass();
            }
            return vessel_component.get_mass();
        }

        error!("Request to get mass of entity without orbitable or vessel components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    /// # Panics
    /// Panics if entity does not have a mass
    pub fn get_mass_at_time(&self, entity: Entity, time: f64) -> f64 {
        if let Some(orbitable_component) = self.try_get_orbitable_component(entity) {
            return orbitable_component.get_mass()
        }

        if let Some(vessel_component) = self.try_get_vessel_component(entity) {

            // find last burn before time if it exists
            for segment in self.get_path_component(entity).get_segments().iter().flatten().rev() {
                if segment.start_time() > time {
                    continue;
                }

                if let Segment::Burn(burn) = segment {
                    if time > segment.start_time() && time < segment.end_time() {
                        // The requested time is within the burn
                        return burn.point_at_time(time).get_mass();
                    }
                    
                    // Otherwise, the requested time is after the burn, so return mass at end of burn
                    return burn.end_point().get_mass();
                }
            }
            return vessel_component.get_mass();
        }

        error!("Request to get mass of entity without orbitable or vessel components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }
}