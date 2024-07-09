use std::collections::HashSet;

use api::{explosion::Explosion, time::TimeStep};
use components::vessel_component::VesselComponent;
use serde::{Deserialize, Serialize};
use systems::update_warp::TimeWarp;
use self::{components::{name_component::NameComponent, orbitable_component::OrbitableComponent, atmosphere_component::AtmosphereComponent, path_component::PathComponent, ComponentType}, storage::{component_storage::ComponentStorage, entity_allocator::{Entity, EntityAllocator}, entity_builder::EntityBuilder}};

pub const SEGMENTS_TO_PREDICT: usize = 3;

pub mod components;
mod debug;
pub mod api;
pub mod storage;
mod systems;
mod util;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    entity_allocator: EntityAllocator,
    name_components: ComponentStorage<NameComponent>,
    orbitable_components: ComponentStorage<OrbitableComponent>,
    atmosphere_components: ComponentStorage<AtmosphereComponent>,
    path_components: ComponentStorage<PathComponent>,
    vessel_components: ComponentStorage<VesselComponent>,
    time: f64,
    time_step: TimeStep,
    warp: Option<TimeWarp>,
    explosions_started_this_frame: Vec<Explosion>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            entity_allocator: EntityAllocator::default(),
            name_components: ComponentStorage::default(),
            orbitable_components: ComponentStorage::default(),
            atmosphere_components: ComponentStorage::default(),
            path_components: ComponentStorage::default(),
            vessel_components: ComponentStorage::default(),
            time: 0.0,
            time_step: TimeStep::Level{ level: 1, paused: false },
            warp: None,
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

    pub fn update(&mut self, dt: f64) {
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
        self.update_fuel();
        self.update_trajectory();
        self.update_guidance();
    }

    pub fn entities(&self, mut with_component_types: Vec<ComponentType>) -> HashSet<Entity> {
        let mut entities = self.entity_allocator.entities().clone();
        while let Some(component_type) = with_component_types.pop() {
            let other_entities = match component_type {
                ComponentType::NameComponent => self.name_components.entities(),
                ComponentType::OrbitableComponent => self.orbitable_components.entities(),
                ComponentType::AtmosphereComponent => self.atmosphere_components.entities(),
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
            atmosphere_component,
            path_component,
            vessel_component,
        } = entity_builder;
        let entity = self.entity_allocator.allocate();
        self.name_components.set(entity, name_component);
        self.orbitable_components.set(entity, orbitable_component);
        self.atmosphere_components.set(entity, atmosphere_component);
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

    pub fn entity_exists(&self, entity: Entity) -> bool {
        self.entity_allocator.entities().contains(&entity)
    }
    
    pub fn warp(&self) -> Option<&TimeWarp> {
        self.warp.as_ref()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{components::{name_component::NameComponent, ComponentType}, storage::entity_builder::EntityBuilder};

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