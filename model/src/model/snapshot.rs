use nalgebra_glm::{vec2, DVec2};

use crate::{components::{orbitable_component::OrbitableComponentPhysics, path_component::{burn::Burn, guidance::Guidance, orbit::Orbit, segment::Segment, turn::Turn}, vessel_component::faction::Faction}, storage::entity_allocator::Entity};

use super::{closest_approach, encounters::{self, Encounter}, state_query::StateQuery, Model};

pub struct Snapshot {
    model: *const Model,
    time: f64,
    observer: Option<Faction>,
}

impl Snapshot {
    pub fn new(model: &Model, time: f64, observer: Option<Faction>) -> Self {
        let model = model as *const Model;
        Self { model, time, observer }
    }

    pub fn model(&self) -> &Model {
        unsafe {
            &*self.model
        }
    }

    pub fn segment_starting_now(&self, entity: Entity) -> &Segment {
        if let Some(orbitable_component) = self.model().try_orbitable_component(entity) {
            if let OrbitableComponentPhysics::Orbit(orbit) = orbitable_component.physics() {
                return orbit;
            };
            panic!("Attempt to get segment of stationary orbitable")
        }
        if let Some(observer) = self.observer {
            if !observer.has_intel_for(self.model().faction(entity)) {
                return self.model()
                    .path_component(entity)
                    .perceived_segment_starting_at_time(self.time)
                    .expect("No segment starts at the given time");
            }
        }
        self.model()
            .path_component(entity)
            .future_segment_starting_at_time(self.time)
            .expect("No segment starts at the given time")
    }

    pub fn orbit_starting_now(&self, entity: Entity) -> &Orbit {
        self.segment_starting_now(entity)
            .as_orbit()
            .expect("No orbit starts at the given time")
    }

    pub fn burn_starting_now(&self, entity: Entity) -> &Burn {
        self.segment_starting_now(entity)
            .as_burn()
            .expect("No burn starts at the given time")
    }

    pub fn turn_starting_now(&self, entity: Entity) -> &Turn {
        self.segment_starting_now(entity)
            .as_turn()
            .expect("No turn starts at the given time")
    }

    pub fn guidance_starting_now(&self, entity: Entity) -> &Guidance {
        self.segment_starting_now(entity)
            .as_guidance()
            .expect("No guidance starts at the given time")
    }
}

impl StateQuery for Snapshot {
    /// Returns the future segments starting at the CURRENT time, not the snapshot time
    /// This is because we'd have to clone the entire segment array to make the necessary
    /// adjustment to the first segment so that it has the correct current time, which could be 
    /// extremely expensive.
    fn future_segments(&self, entity: Entity) -> Vec<&Segment> {
        if let Some(orbitable_component) = self.model().try_orbitable_component(entity) {
            if let OrbitableComponentPhysics::Orbit(orbit) = orbitable_component.physics() {
                return vec![orbit];
            };
            panic!("Attempt to get segment of stationary orbitable")
        }
        if let Some(observer) = self.observer {
            if !observer.has_intel_for(self.model().faction(entity)) {
                return self.model().path_component(entity).perceived_segments();
            }
        }
        self.model().future_segments(entity)
    }

    /// Returns the future orbits starting at the CURRENT time, not the snapshot time
    fn future_orbits(&self, entity: Entity) -> Vec<&Orbit> {
        self.future_segments(entity)
            .iter()
            .filter_map(|segment| segment.as_orbit())
            .collect()
    }

    /// Returns the future burns starting at the CURRENT time, not the snapshot time
    fn future_burns(&self, entity: Entity) -> Vec<&Burn> {
        self.future_segments(entity)
            .iter()
            .filter_map(|segment| segment.as_burn())
            .collect()
    }

    /// Returns the future turns starting at the CURRENT time, not the snapshot time
    fn future_turns(&self, entity: Entity) -> Vec<&Turn> {
        self.future_segments(entity)
            .iter()
            .filter_map(|segment| segment.as_turn())
            .collect()
    }

    /// Returns the future guidances starting at the CURRENT time, not the snapshot time
    fn future_guidances(&self, entity: Entity) -> Vec<&Guidance> {
        self.future_segments(entity)
            .iter()
            .filter_map(|segment| segment.as_guidance())
            .collect()
    }

    /// Returns the current segment at the snapshot time, but the current time within the segment
    /// will not be the snapshot time.
    /// This is because we'd have to clone the segment to make the adjustment so that it has 
    /// the correct current time, which could be extremely expensive.
    fn segment(&self, entity: Entity) -> &Segment {
        if let Some(orbitable_component) = self.model().try_orbitable_component(entity) {
            if let OrbitableComponentPhysics::Orbit(orbit) = orbitable_component.physics() {
                return orbit;
            };
            panic!("Attempt to get segment of stationary orbitable")
        }
        if let Some(observer) = self.observer {
            if !observer.has_intel_for(self.model().faction(entity)) {
                return self.model().path_component(entity).perceived_segment_at_time(self.time);
            }
        }
        self.model().path_component(entity).future_segment_at_time(self.time)
    }

    /// Returns the current segment at the snapshot time, but the current time within the segment
    /// will not be the snapshot time.
    fn orbit(&self, entity: Entity) -> &Orbit {
        self.segment(entity)
            .as_orbit()
            .expect("No orbit exists at the given time")
    }

    /// Returns the current segment at the snapshot time, but the current time within the segment
    /// will not be the snapshot time.
    fn burn(&self, entity: Entity) -> &Burn {
        self.segment(entity)
            .as_burn()
            .expect("No burn exists at the given time")
    }

    /// Returns the current segment at the snapshot time, but the current time within the segment
    /// will not be the snapshot time.
    fn turn(&self, entity: Entity) -> &Turn {
        self.segment(entity)
            .as_turn()
            .expect("No turn exists at the given time")
    }

    /// Returns the current segment at the snapshot time, but the current time within the segment
    /// will not be the snapshot time.
    fn guidance(&self, entity: Entity) -> &Guidance {
        self.segment(entity)
            .as_guidance()
            .expect("No guidance exists at the given time")
    }

    fn parent(&self, entity: Entity) -> Option<Entity> {
        if let Some(orbitable_component) = self.model().try_orbitable_component(entity) {
            if orbitable_component.physics().is_stationary() {
                return None;
            }
        }
        Some(self.segment(entity).parent())
    }

    fn target(&self, entity: Entity) -> Option<Entity> {
        if let Some(observer) = self.observer {
            if !observer.has_intel_for(self.model().faction(entity)) {
                return None;
            }
        }
        self.model().target(entity)
    }

    fn rotation(&self, entity: Entity) -> f64 {
        if self.model().has_orbitable_component(entity) {
            return 0.0;
        }
        self.segment(entity).rotation_at_time(self.time)
    }

    fn displacement(&self, entity: Entity, other_entity: Entity) -> DVec2 {
        let entity_absolute_position = self.absolute_position(entity);
        let other_entity_absolute_position = self.absolute_position(other_entity);
        entity_absolute_position - other_entity_absolute_position
    }

    fn distance(&self, entity: Entity, other_entity: Entity) -> f64 {
        self.displacement(entity, other_entity).magnitude()
    }

    fn surface_altitude(&self, entity: Entity) -> f64 {
        let parent = self.parent(entity);
        let parent_radius = self.model().orbitable_component(parent.unwrap()).radius();
        self.position(entity).magnitude() - parent_radius
    }

    fn position(&self, entity: Entity) -> DVec2 {
        self.segment(entity).position_at_time(self.time)
    }

    fn absolute_position(&self, entity: Entity) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute position snapshot");
        if let Some(orbitable_component) = self.model().try_orbitable_component(entity) {
            if let OrbitableComponentPhysics::Stationary(position) = orbitable_component.physics() {
                // Base case
                return *position;
            }
        }
        let segment = self.segment(entity);
        self.absolute_position(segment.parent()) + segment.position_at_time(self.time)
    }

    fn velocity(&self, entity: Entity) -> DVec2 {
        self.segment(entity).velocity_at_time(self.time)
    }

    fn absolute_velocity(&self, entity: Entity) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute velocity snapshot");
        if let Some(orbitable_component) = self.model().try_orbitable_component(entity) {
            if orbitable_component.physics().is_stationary() {
                return vec2(0.0, 0.0);
            }
        }
        let segment = self.segment(entity);
        self.absolute_velocity(segment.parent()) + segment.velocity_at_time(self.time)
    }

    fn relative_velocity(&self, entity: Entity, other_entity: Entity) -> DVec2 {
        let entity_velocity = self.absolute_velocity(entity);
        let other_entity_velocity = self.absolute_velocity(other_entity);
        entity_velocity - other_entity_velocity
    }

    fn relative_speed(&self, entity: Entity, other_entity: Entity) -> f64 {
        self.relative_velocity(entity, other_entity).magnitude()
    }

    fn mass(&self, entity: Entity) -> f64 {
        if let Some(observer) = self.observer {
            assert!(observer.has_intel_for(self.model().faction(entity)));
        }
        if let Some(orbitable_component) = self.model().try_orbitable_component(entity) {
            return orbitable_component.mass();
        }
        self.segment(entity).mass_at_time(self.time)
    }

    fn fuel_kg(&self, entity: Entity) -> f64 {
        if let Some(observer) = self.observer {
            assert!(observer.has_intel_for(self.model().faction(entity)));
        }
        if let Some(fuel_kg) = self.model().path_component(entity).fuel_kg_at_time(self.time) {
            return fuel_kg
        }
        self.model().fuel_kg(entity)
    }

    fn end_fuel(&self, entity: Entity) -> Option<f64> {
        if let Some(observer) = self.observer {
            if !observer.has_intel_for(self.model().faction(entity)){
                return None;
            }
        }
        if let Some(path_component) = self.model().try_path_component(entity) {
            if let Some(end_fuel_kg) = path_component.end_fuel_kg() {
                return Some(end_fuel_kg);
            }
        }
        Some(self.model().vessel_component(entity).fuel_kg())
    }

    fn end_dv(&self, entity: Entity) -> Option<f64> {
        if let Some(observer) = self.observer {
            if !observer.has_intel_for(self.model().faction(entity)){
                return None;
            }
        }
        if let Some(path_component) = self.model().try_path_component(entity) {
            if let Some(end_dv) = path_component.end_dv() {
                return Some(end_dv);
            }
        }
        Some(self.model().vessel_component(entity).dv())
    }

    fn find_next_closest_approach(&self, entity_a: Entity, entity_b: Entity) -> Option<f64> {
        closest_approach::find_next_closest_approach(self.model(), entity_a, entity_b, self.time, self.observer)
    }

    fn find_next_two_closest_approaches(&self, entity_a: Entity, entity_b: Entity) -> (Option<f64>, Option<f64>) {
        closest_approach::find_next_two_closest_approaches(self.model(), entity_a, entity_b, self.time, self.observer)
    }

    fn future_encounters(&self, entity: Entity) -> Vec<Encounter> {
        encounters::future_encounters(self.model(), entity, self.time, self.observer)
    }
}

impl Model {
    pub fn snapshot(&self, time: f64, observer: Option<Faction>) -> Snapshot {
        Snapshot::new(self, time, observer)
    }

    pub fn snapshot_now(&self) -> Snapshot {
        Snapshot::new(self, self.time, None)
    }

    pub fn snapshot_now_observe(&self, observer: Faction) -> Snapshot {
        Snapshot::new(self, self.time, Some(observer))
    }

    pub fn snapshot_now_observe_maybe(&self, observer: Option<Faction>) -> Snapshot {
        Snapshot::new(self, self.time, observer)
    }

    pub fn snapshot_at(&self, time: f64) -> Snapshot {
        Snapshot::new(self, time, None)
    }

    pub fn snapshot_at_observe(&self, time: f64, observer: Faction) -> Snapshot {
        Snapshot::new(self, time, Some(observer))
    }

    pub fn snapshot_at_observe_maybe(&self, time: f64, observer: Option<Faction>) -> Snapshot {
        Snapshot::new(self, time, observer)
    }
}
