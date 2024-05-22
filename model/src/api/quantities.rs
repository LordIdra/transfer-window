use nalgebra_glm::{vec2, DVec2};

use crate::{components::orbitable_component::OrbitableComponentPhysics, storage::entity_allocator::Entity, Model};

impl Model {
    /// # Panics
    /// Panics if entity does not have a position
    pub fn position(&self, entity: Entity) -> DVec2 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.position();
        }

        if let Some(path_component) = self.try_path_component(entity) {
            return path_component.current_segment().current_position();
        }

        panic!("Request to get position of entity without path or orbitable components");
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn position_at_time(&self, entity: Entity, time: f64) -> DVec2 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.position();
        }

        if let Some(path_component) = self.try_path_component(entity) {
            return path_component.future_segment_at_time(time).position_at_time(time);
        }

        panic!("Request to get position of entity without path or orbitable components");
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn absolute_position(&self, entity: Entity) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute position");
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(position) => *position,
                OrbitableComponentPhysics::Orbit(orbit) => self.absolute_position(orbit.parent()) + orbit.current_point().position(),
            }
        }

        if let Some(path_component) = self.try_path_component(entity) {
            let current_segment = path_component.current_segment();
            return self.absolute_position(current_segment.parent()) + current_segment.current_position();
        }

        panic!("Request to get absolute position of entity without path or orbitable components");
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn absolute_position_at_time(&self, entity: Entity, time: f64) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute position at time");
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(position) => *position,
                OrbitableComponentPhysics::Orbit(orbit) => self.absolute_position_at_time(orbit.parent(), time) + orbit.point_at_time(time).position(),
            }
        }

        if let Some(path_component) = self.try_path_component(entity) {
            let segment = path_component.future_segment_at_time(time);
            return self.absolute_position_at_time(segment.parent(), time) + segment.position_at_time(time);
        }

        panic!("Request to get absolute position of entity without path or orbitable components");
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn velocity(&self, entity: Entity) -> DVec2 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
                OrbitableComponentPhysics::Orbit(orbit) => orbit.current_point().velocity(),
            }
        }

        if let Some(path_component) = self.try_path_component(entity) {
            return path_component.current_segment().current_velocity();
        }

        panic!("Request to get velocity of entity without path or orbitable components");
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn velocity_at_time(&self, entity: Entity, time: f64) -> DVec2 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
                OrbitableComponentPhysics::Orbit(orbit) => orbit.point_at_time(time).velocity(),
            }
        }

        if let Some(path_component) = self.try_path_component(entity) {
            let segment_at_time = path_component.future_segment_at_time(time);
            return segment_at_time.velocity_at_time(time);
        }

        panic!("Request to get velocity of entity without path or orbitable components");
    }

    /// # Panics
    /// Panics if entity does not have a velocity
    pub fn absolute_velocity(&self, entity: Entity) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute velocity");
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
                OrbitableComponentPhysics::Orbit(orbit) => self.absolute_velocity(orbit.parent()) + orbit.current_point().velocity(),
            }
        }

        if let Some(path_component) = self.try_path_component(entity) {
            let current_segment = path_component.current_segment();
            return self.absolute_velocity(current_segment.parent()) + current_segment.current_velocity();
        }

        panic!("Request to get absolute velocity of entity without path or orbitable components");
    }

    /// # Panics
    /// Panics if entity does not have a velocity
    pub fn absolute_velocity_at_time(&self, entity: Entity, time: f64) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute velocity at time");
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
                OrbitableComponentPhysics::Orbit(orbit) => self.absolute_velocity_at_time(orbit.parent(), time) + orbit.point_at_time(time).velocity(),
            }
        }

        if let Some(path_component) = self.try_path_component(entity) {
            let segment = path_component.future_segment_at_time(time);
            return self.absolute_velocity_at_time(segment.parent(), time) + segment.velocity_at_time(time);
        }

        panic!("Request to get absolute velocity of entity without path or orbitable components");
    }

    /// # Panics
    /// Panics if entity does not have a mass
    pub fn mass(&self, entity: Entity) -> f64 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.mass();
        }

        if let Some(path_component) = self.try_path_component(entity) {
            return path_component.current_mass();
        }

        panic!("Request to get mass of entity without orbitable or vessel components");
    }

    /// # Panics
    /// Panics if entity does not have a mass
    pub fn mass_at_time(&self, entity: Entity, time: f64) -> f64 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.mass();
        }

        if let Some(path_component) = self.try_path_component(entity) {
            return path_component.mass_at_time(time);
        }

        panic!("Request to get mass of entity without orbitable or vessel components");
    }

    /// Returns none if the entity does not have an engine
    /// # Panics
    /// Panics if the entity does not have a vessel component
    pub fn final_dv(&self, entity: Entity) -> Option<f64> {
        if let Some(vessel_component) = self.try_vessel_component(entity) {
            if let Some(rocket_equation_function) = self.path_component(entity).final_rocket_equation_function() {
                return Some(rocket_equation_function.remaining_dv());
            }
            return vessel_component.dv();
        }

        panic!("Request to get dv of entity without vessel component");
    }

    pub fn distance(&self, entity: Entity, other_entity: Entity) -> f64 {
        (self.absolute_position(entity) - self.absolute_position(other_entity)).magnitude()
    }

    pub fn distance_at_time(&self, entity: Entity, other_entity: Entity, time: f64) -> f64 {
        (self.absolute_position_at_time(entity, time) - self.absolute_position_at_time(other_entity, time)).magnitude()
    }
}