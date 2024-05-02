use std::collections::HashSet;

use components::vessel_component::VesselComponent;
use serde::{Deserialize, Serialize};
use systems::{fuel_depletion, time::{self, TimeStep}, trajectory_update, warp_update_system::{self, TimeWarp}};

use self::{components::{name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::TrajectoryComponent, ComponentType}, storage::{component_storage::ComponentStorage, entity_allocator::{Entity, EntityAllocator}, entity_builder::EntityBuilder}};

pub const SEGMENTS_TO_PREDICT: usize = 4;

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
    stationary_components: ComponentStorage<StationaryComponent>,
    trajectory_components: ComponentStorage<TrajectoryComponent>,
    vessel_components: ComponentStorage<VesselComponent>,
    time: f64,
    time_step: TimeStep,
    warp: Option<TimeWarp>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            entity_allocator: EntityAllocator::default(),
            name_components: ComponentStorage::default(),
            orbitable_components: ComponentStorage::default(),
            trajectory_components: ComponentStorage::default(),
            stationary_components: ComponentStorage::default(),
            vessel_components: ComponentStorage::default(),
            time: 0.0,
            time_step: TimeStep::Level{ level: 1, paused: false },
            warp: None,
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
        fuel_depletion::update_fuel_depletion(self, dt);
        warp_update_system::update(self, dt);
        time::update(self, dt);
        trajectory_update::update(self, dt);
    }

    pub fn get_entities(&self, mut with_component_types: Vec<ComponentType>) -> HashSet<Entity> {
        let mut entities = self.entity_allocator.get_entities().clone();
        while let Some(component_type) = with_component_types.pop() {
            let other_entities = match component_type {
                ComponentType::NameComponent => self.name_components.get_entities(),
                ComponentType::OrbitableComponent => self.orbitable_components.get_entities(),
                ComponentType::StationaryComponent => self.stationary_components.get_entities(),
                ComponentType::TrajectoryComponent => self.trajectory_components.get_entities(),
                ComponentType::VesselComponent => self.vessel_components.get_entities(),
            };
            entities.retain(|entity| other_entities.contains(entity));
        }
        entities
    }

    pub fn allocate(&mut self, entity_builder: EntityBuilder) -> Entity {
        let EntityBuilder {
            name_component,
            orbitable_component,
            stationary_component,
            trajectory_component,
            vessel_component,
        } = entity_builder;
        let entity = self.entity_allocator.allocate();
        self.name_components.set(entity, name_component);
        self.orbitable_components.set(entity, orbitable_component);
        self.stationary_components.set(entity, stationary_component);
        self.trajectory_components.set(entity, trajectory_component);
        self.vessel_components.set(entity, vessel_component);
        entity
    }

    pub fn deallocate(&mut self, entity: Entity) {
        self.entity_allocator.deallocate(entity);
        self.name_components.remove_if_exists(entity);
        self.orbitable_components.remove_if_exists(entity);
        self.stationary_components.remove_if_exists(entity);
        self.trajectory_components.remove_if_exists(entity);
        self.vessel_components.remove_if_exists(entity);
    }

    pub fn entity_exists(&self, entity: Entity) -> bool {
        self.entity_allocator.get_entities().contains(&entity)
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
        let builder1 = EntityBuilder::default().with_name_component(NameComponent::new("oh no".to_string()));
        let builder2 = EntityBuilder::default();
        let e1 = model.allocate(builder1);
        let e2 = model.allocate(builder2);

        let mut expected = HashSet::new();
        expected.insert(e1);
        expected.insert(e2);
        assert!(model.get_entities(vec![]) == expected);

        let mut expected = HashSet::new();
        expected.insert(e1);
        assert!(model.get_entities(vec![ComponentType::NameComponent]) == expected);
    }
}