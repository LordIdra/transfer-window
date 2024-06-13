use std::collections::VecDeque;

use nalgebra_glm::{vec2, DVec2};

use crate::{components::{orbitable_component::OrbitableComponentPhysics, path_component::{orbit::Orbit, segment::Segment}, vessel_component::Faction}, storage::entity_allocator::Entity, Model};

impl Model {
    /// NOT safe to call for orbitables
    pub fn future_segments(&self, entity: Entity, observer: Option<Faction>) -> VecDeque<Segment> {
        if let Some(observer) = observer {
            if !observer.has_intel_for(self.vessel_component(entity).faction()) {
                return self.compute_perceived_path(entity);
            }
        }
        self.path_component(entity).future_segments().clone() // TODO get rid of this clone and implement proper caching
    }

    /// NOT safe to call for orbitables
    pub fn future_orbits(&self, entity: Entity, observer: Option<Faction>) -> Vec<Orbit> {
        if let Some(observer) = observer {
            if !observer.has_intel_for(self.vessel_component(entity).faction()) {
                return self.compute_perceived_path(entity).iter()
                    .map(|segment| segment.as_orbit().unwrap().clone())
                    .collect();
            }
        }
        self.path_component(entity).future_orbits().iter().cloned().cloned().collect() // TODO get rid of this clone and implement proper caching
    }

    /// NOT Safe to call for orbitables
    pub fn current_segment(&self, entity: Entity) -> &Segment {
        if let Some(path_component) = self.try_path_component(entity) {
            return path_component.current_segment();
        }
        panic!("Attempt to get current segment of entity without path or orbitable component")
    }

    /// NOT Safe to call for orbitables
    pub fn current_orbit(&self, entity: Entity) -> &Orbit {
        self.current_segment(entity)
            .as_orbit()
            .expect("Segment is not orbit")
    }
    
    /// NOT Safe to call for orbitables
    pub fn segment_at_time(&self, entity: Entity, time: f64, observer: Option<Faction>) -> Segment {
        if let Some(observer) = observer {
            if !observer.has_intel_for(self.vessel_component(entity).faction()) {
                // The reason we clamp to the end time is because encounter prediction is nondeterministic
                // So if we call encounter prediction one frame, get the time, and store the result, the
                // same call the next frame might be very slightly sooner. Then suppose we feed the result into
                // this function... oh, look, we got a panic because the stored time is slightly after the 
                // end segment. Yes this is a stupid solution but I don't know how to better solve it
                let perceived_path = self.compute_perceived_path(entity);
                let time = f64::min(time, perceived_path.back().unwrap().end_time());
                for segment in perceived_path {
                    if time >= segment.start_time() && time <= segment.end_time() {
                        return segment;
                    }
                }
                panic!("No segment exists at the given time")
            }
        }
        self.path_component(entity).future_segment_at_time(time).clone() // TODO get rid of this clone and implement proper caching
    }

    /// NOT Safe to call for orbitables
    pub fn orbit_at_time(&self, entity: Entity, time: f64, observer: Option<Faction>) -> Orbit {
        self.segment_at_time(entity, time, observer)
            .as_orbit()
            .expect("Segment at time is not orbit")
            .clone() //TODO YEET
    }

    pub fn parent(&self, entity: Entity) -> Option<Entity> {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.orbit().map(Orbit::parent);
        }

        if let Some(path_component) = self.try_path_component(entity) {
            return Some(path_component.current_segment().parent());
        }

        None
    }

    pub fn parent_at_time(&self, entity: Entity, time: f64, observer: Option<Faction>) -> Option<Entity> {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(_) => None,
                OrbitableComponentPhysics::Orbit(orbit) => Some(orbit.parent()),
            }
        }
        Some(self.segment_at_time(entity, time, observer).parent())
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn position(&self, entity: Entity) -> DVec2 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.position();
        }
        self.current_segment(entity).current_position()
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn position_at_time(&self, entity: Entity, time: f64, observer: Option<Faction>) -> DVec2 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.position();
        }
        self.segment_at_time(entity, time, observer).position_at_time(time)
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
        let segment = self.current_segment(entity);
        self.absolute_position(segment.parent()) + segment.current_position()
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn absolute_position_at_time(&self, entity: Entity, time: f64, observer: Option<Faction>) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute position at time");
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(position) => *position,
                OrbitableComponentPhysics::Orbit(orbit) => self.absolute_position_at_time(orbit.parent(), time, observer) + orbit.point_at_time(time).position(),
            }
        }
        let segment = self.segment_at_time(entity, time, observer);
        self.absolute_position_at_time(segment.parent(), time, observer) + segment.position_at_time(time)
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
        self.current_segment(entity).current_velocity()
    }

    /// # Panics
    /// Panics if entity does not have a position
    pub fn velocity_at_time(&self, entity: Entity, time: f64, observer: Option<Faction>) -> DVec2 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
                OrbitableComponentPhysics::Orbit(orbit) => orbit.point_at_time(time).velocity(),
            }
        }
        self.segment_at_time(entity, time, observer).velocity_at_time(time)
    }

    /// # Panics
    /// Panics if entity does not have a velocity
    pub fn absolute_velocity(&self, entity: Entity, observer: Option<Faction>) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute velocity");
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
                OrbitableComponentPhysics::Orbit(orbit) => self.absolute_velocity(orbit.parent(), observer) + orbit.current_point().velocity(),
            }
        }
        let segment = self.current_segment(entity);
        self.absolute_velocity(segment.parent(), observer) + segment.current_velocity()
    }

    /// # Panics
    /// Panics if entity does not have a velocity
    pub fn absolute_velocity_at_time(&self, entity: Entity, time: f64, observer: Option<Faction>) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute velocity at time");
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return match orbitable_component.physics() {
                OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
                OrbitableComponentPhysics::Orbit(orbit) => self.absolute_velocity_at_time(orbit.parent(), time, observer) + orbit.point_at_time(time).velocity(),
            }
        }
        let segment = self.segment_at_time(entity, time, observer);
        self.absolute_velocity_at_time(segment.parent(), time, observer) + segment.velocity_at_time(time)
    }

    /// # Panics
    /// Panics if entity does not have a mass
    pub fn mass(&self, entity: Entity) -> f64 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.mass();
        }
        self.current_segment(entity).current_mass()
    }

    /// # Panics
    /// Panics if entity does not have a mass
    pub fn mass_at_time(&self, entity: Entity, time: f64, observer: Option<Faction>) -> f64 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.mass();
        }
        self.segment_at_time(entity, time, observer).current_mass()
    }

    /// Returns none if the entity does not have an engine
    /// # Panics
    /// Panics if the entity does not have a vessel component
    pub fn final_dv(&self, entity: Entity) -> Option<f64> {
        if let Some(vessel_component) = self.try_vessel_component(entity) {
            return match self.path_component(entity).final_rocket_equation_function() {
                Some(rocket_equation_function) => Some(rocket_equation_function.remaining_dv()),
                None => vessel_component.dv(),
            };
        }
        panic!("Attempt to get dv of entity without vessel component");
    }

    pub fn distance_at_time(&self, entity: Entity, other_entity: Entity, time: f64, observer: Option<Faction>) -> f64 {
        (self.absolute_position_at_time(entity, time, observer) - self.absolute_position_at_time(other_entity, time, observer)).magnitude()
    }
}