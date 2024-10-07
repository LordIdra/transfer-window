use std::{collections::HashSet, sync::Mutex};

use encounters::Encounter;
use explosion::Explosion;
use nalgebra_glm::{vec2, DVec2};
use serde::{Deserialize, Serialize};
use state_query::StateQuery;
use story_event::StoryEvent;
use time::{TimeStep, TimeWarp};

use crate::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics}, path_component::{burn::Burn, guidance::Guidance, orbit::Orbit, segment::Segment, turn::Turn, PathComponent}, vessel_component::{faction::Faction, VesselComponent}, ComponentType}, storage::{component_storage::ComponentStorage, entity_allocator::{Entity, EntityAllocator}, entity_builder::EntityBuilder}};

pub const SEGMENTS_TO_PREDICT: usize = 3;

pub mod closest_approach;
pub mod closest_point;
pub mod component;
pub mod docking;
pub mod encounters;
pub mod explosion;
pub mod segment;
pub mod snapshot;
pub mod state_query;
pub mod story_event;
pub mod time;
pub mod timeline;
pub mod trajectories;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    entity_allocator: EntityAllocator,
    name_components: ComponentStorage<NameComponent>,
    orbitable_components: ComponentStorage<OrbitableComponent>,
    path_components: ComponentStorage<PathComponent>,
    vessel_components: ComponentStorage<VesselComponent>,
    story_events: Mutex<Vec<StoryEvent>>,
    time: f64,
    time_step: TimeStep,
    warp: Option<TimeWarp>,
    force_paused: bool,
    explosions_started_this_frame: Vec<Explosion>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            entity_allocator: EntityAllocator::default(),
            name_components: ComponentStorage::default(),
            orbitable_components: ComponentStorage::default(),
            path_components: ComponentStorage::default(),
            vessel_components: ComponentStorage::default(),
            story_events: Mutex::new(vec![]),
            time: 0.0,
            time_step: TimeStep::Level{ level: 1, paused: false },
            warp: None,
            force_paused: false,
            explosions_started_this_frame: vec![],
        }
    }
}

impl Model {

    /// # Errors
    /// Forwards serde deserialization error if deserialization fails
    pub fn deserialize(serialized: &str) -> Result<Self, String> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Deserialize");
        match serde_json::from_str(serialized) {
            Ok(model) => Ok(model),
            Err(error) => Err(error.to_string()),
        }
    }

    /// # Errors
    /// Forwards serde serialization error if serialization fails
    pub fn serialize(&self) -> Result<String, String> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Serialize");
        match serde_json::to_string(self) {
            Ok(serialized) => Ok(serialized),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn update(&mut self, dt: f64) -> Vec<StoryEvent> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Model update");
        self.explosions_started_this_frame.clear();
        self.update_warp(dt);
        self.update_time(dt);
        self.update_timeline();
        self.update_objects();
        self.update_target();
        self.update_stations(dt);
        self.update_launcher_cooldown(dt);
        self.update_trajectory();
        self.update_guidance();
        let story_events = self.story_events.lock().unwrap().clone();
        self.story_events = Mutex::new(vec![]);
        story_events
    }

    pub fn entities(&self, mut with_component_types: Vec<ComponentType>) -> HashSet<Entity> {
        let mut entities = self.entity_allocator.entities().clone();
        while let Some(component_type) = with_component_types.pop() {
            let other_entities = match component_type {
                ComponentType::NameComponent => self.name_components.entities(),
                ComponentType::OrbitableComponent => self.orbitable_components.entities(),
                ComponentType::PathComponent => self.path_components.entities(),
                ComponentType::VesselComponent => self.vessel_components.entities(),
            };
            entities.retain(|entity| other_entities.contains(entity));
        }
        entities
    }

    pub fn allocate(&mut self, entity_builder: EntityBuilder) -> Entity {
        let EntityBuilder {
            name_component,
            orbitable_component,
            path_component,
            vessel_component,
        } = entity_builder;
        let entity = self.entity_allocator.allocate();
        self.name_components.set(entity, name_component);
        self.orbitable_components.set(entity, orbitable_component);
        self.path_components.set(entity, path_component);
        self.vessel_components.set(entity, vessel_component);
        entity
    }

    pub fn deallocate(&mut self, entity: Entity) {
        self.entity_allocator.deallocate(entity);
        self.name_components.remove_if_exists(entity);
        self.orbitable_components.remove_if_exists(entity);
        self.path_components.remove_if_exists(entity);
        self.vessel_components.remove_if_exists(entity);
    }

    pub fn exists(&self, entity: Entity) -> bool {
        self.entity_allocator.entities().contains(&entity)
    }

    pub fn faction(&self, entity: Entity) -> Faction {
        self.vessel_component(entity).faction()
    }
}

impl StateQuery for Model {
    fn future_segments(&self, entity: Entity) -> Vec<&Segment> {
        self.path_component(entity).future_segments()
    }

    fn future_orbits(&self, entity: Entity) -> Vec<&Orbit> {
        self.path_component(entity).future_orbits()
    }

    fn future_burns(&self, entity: Entity) -> Vec<&Burn> {
        self.path_component(entity).future_burns()
    }

    fn future_turns(&self, entity: Entity) -> Vec<&Turn> {
        self.path_component(entity).future_turns()
    }

    fn future_guidances(&self, entity: Entity) -> Vec<&Guidance> {
        self.path_component(entity).future_guidances()
    }

    fn segment(&self, entity: Entity) -> &Segment {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            if let OrbitableComponentPhysics::Orbit(segment) = orbitable_component.physics() {
                return segment;
            }
            panic!("Attempted to get segment of stationary orbitable")
        }
        self.path_component(entity).current_segment()
    }

    fn orbit(&self, entity: Entity) -> &Orbit {
        self.segment(entity)
            .as_orbit()
            .expect("Segment is not orbit")
    }

    fn burn(&self, entity: Entity) -> &Burn {
        self.segment(entity)
            .as_burn()
            .expect("Segment is not orbit")
    }

    fn turn(&self, entity: Entity) -> &Turn {
        self.segment(entity)
            .as_turn()
            .expect("Segment is not orbit")
    }

    fn guidance(&self, entity: Entity) -> &Guidance {
        self.segment(entity)
            .as_guidance()
            .expect("Segment is not orbit")
    }

    fn parent(&self, entity: Entity) -> Option<Entity> {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.orbit().map(Orbit::parent);
        }
        if let Some(path_component) = self.try_path_component(entity) {
            return Some(path_component.current_segment().parent());
        }
        None
    }

    fn target(&self, entity: Entity) -> Option<Entity> {
        self.try_vessel_component(entity)?.target()
    }

    fn rotation(&self, entity: Entity) -> f64 {
        if self.try_orbitable_component(entity).is_some() {
            return 0.0;
        }
        self.segment(entity).current_rotation()
    }

    fn surface_altitude(&self, entity: Entity) -> f64 {
        let parent = self.parent(entity).expect("Attempt to get surface altitude of entity without parent");
        let parent_radius = self.orbitable_component(parent).radius();
        self.position(entity).magnitude() - parent_radius
    }

    fn position(&self, entity: Entity) -> DVec2 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.position();
        }
        self.segment(entity).current_position()
    }

    fn absolute_position(&self, entity: Entity) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute position");
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            if let OrbitableComponentPhysics::Stationary(position) = orbitable_component.physics() {
                // Base case
                return *position;
            }
        }
        let segment = self.segment(entity);
        self.absolute_position(segment.parent()) + segment.current_position()
    }

    fn displacement(&self, entity: Entity, other_entity: Entity) -> DVec2 {
        self.absolute_position(entity) - self.absolute_position(other_entity)
    }

    fn distance(&self, entity: Entity, other_entity: Entity) -> f64 {
        self.displacement(entity, other_entity).magnitude()
    }

    fn velocity(&self, entity: Entity) -> DVec2 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.velocity();
        }
        self.segment(entity).current_velocity()
    }

    fn absolute_velocity(&self, entity: Entity) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Absolute velocity");
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            if let OrbitableComponentPhysics::Stationary(_) = orbitable_component.physics() {
                // Base case
                return vec2(0.0, 0.0);
            }
        }
        let segment = self.segment(entity);
        self.absolute_velocity(segment.parent()) + segment.current_velocity()
    }

    fn relative_velocity(&self, entity: Entity, other_entity: Entity) -> DVec2 {
        self.absolute_velocity(entity) - self.absolute_velocity(other_entity)
    }

    fn relative_speed(&self, entity: Entity, other_entity: Entity) -> f64 {
        self.relative_velocity(entity, other_entity).magnitude()
    }

    fn mass(&self, entity: Entity) -> f64 {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.mass();
        }
        self.segment(entity).current_mass()
    }

    fn fuel_kg(&self, entity: Entity) -> f64 {
        self.vessel_component(entity).fuel_kg()
    }

    fn end_fuel(&self, entity: Entity) -> Option<f64> {
        if let Some(path_component) = self.try_path_component(entity) {
            if let Some(end_fuel_kg) = path_component.end_fuel_kg() {
                return Some(end_fuel_kg)
            }
        }
        Some(self.vessel_component(entity).fuel_kg())
    }

    fn end_dv(&self, entity: Entity) -> Option<f64> {
        if let Some(path_component) = self.try_path_component(entity) {
            if let Some(end_dv) = path_component.end_dv() {
                return Some(end_dv)
            }
        }
        if !self.vessel_component(entity).has_engine() {
            return None;
        }
        return Some(self.vessel_component(entity).dv());
    }

    fn find_next_closest_approach(&self, entity_a: Entity, entity_b: Entity) -> Option<f64> {
        closest_approach::find_next_closest_approach(self, entity_a, entity_b, self.time, None)
    }

    fn find_next_two_closest_approaches(&self, entity_a: Entity, entity_b: Entity) -> (Option<f64>, Option<f64>) {
        closest_approach::find_next_two_closest_approaches(self, entity_a, entity_b, self.time, None)
    }
    
    fn future_encounters(&self, entity: Entity) -> Vec<Encounter> {
        encounters::future_encounters(self, entity, self.time, None)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{components::{ComponentType, name_component::NameComponent}, storage::entity_builder::EntityBuilder};

    use super::Model;

    #[test]
    fn test_components() {
        let mut model = Model::default();
        let e1 = model.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new("oh no".to_string())));
        let e2 = model.allocate(EntityBuilder::default());

        let mut expected = HashSet::new();
        expected.insert(e1);
        expected.insert(e2);
        assert!(model.entities(vec![]) == expected);

        let mut expected = HashSet::new();
        expected.insert(e1);
        assert!(model.entities(vec![ComponentType::NameComponent]) == expected);
    }
}
