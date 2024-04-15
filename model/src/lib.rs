use std::collections::HashSet;

use components::vessel_component::VesselComponent;
use serde::{Deserialize, Serialize};
use systems::{time::{self, TimeStep}, trajectory_update, warp_update_system::{self, TimeWarp}};

use self::{components::{mass_component::MassComponent, name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::TrajectoryComponent, ComponentType}, storage::{component_storage::ComponentStorage, entity_allocator::{Entity, EntityAllocator}, entity_builder::EntityBuilder}};

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
    mass_components: ComponentStorage<MassComponent>,
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
            mass_components: ComponentStorage::default(),
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
        warp_update_system::update(self, dt);
        time::update(self, dt);
        trajectory_update::update(self, dt);
    }

    pub fn get_entities(&self, mut with_component_types: Vec<ComponentType>) -> HashSet<Entity> {
        let mut entities = self.entity_allocator.get_entities().clone();
        while let Some(component_type) = with_component_types.pop() {
            let other_entities = match component_type {
                ComponentType::MassComponent => self.mass_components.get_entities(),
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
            mass_component,
            name_component,
            orbitable_component,
            stationary_component,
            trajectory_component,
            vessel_component,
        } = entity_builder;
        let entity = self.entity_allocator.allocate();
        self.mass_components.set(entity, mass_component);
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
    }

    pub fn get_mass_component_mut(&mut self, entity: Entity) -> &mut MassComponent {
        self.mass_components.get_mut(entity)
    }

    pub fn get_mass_component(&self, entity: Entity) -> &MassComponent {
        self.mass_components.get(entity)
    }

    pub fn try_get_mass_component_mut(&mut self, entity: Entity) -> Option<&mut MassComponent> {
        self.mass_components.try_get_mut(entity)
    }

    pub fn try_get_mass_component(&self, entity: Entity) -> Option<&MassComponent> {
        self.mass_components.try_get(entity)
    }

    pub fn get_name_component_mut(&mut self, entity: Entity) -> &mut NameComponent {
        self.name_components.get_mut(entity)
    }

    pub fn get_name_component(&self, entity: Entity) -> &NameComponent {
        self.name_components.get(entity)
    }

    pub fn try_get_name_component_mut(&mut self, entity: Entity) -> Option<&mut NameComponent> {
        self.name_components.try_get_mut(entity)
    }

    pub fn try_get_name_component(&self, entity: Entity) -> Option<&NameComponent> {
        self.name_components.try_get(entity)
    }

    pub fn get_stationary_component_mut(&mut self, entity: Entity) -> &mut StationaryComponent {
        self.stationary_components.get_mut(entity)
    }

    pub fn get_stationary_component(&self, entity: Entity) -> &StationaryComponent {
        self.stationary_components.get(entity)
    }

    pub fn try_get_stationary_component_mut(&mut self, entity: Entity) -> Option<&mut StationaryComponent> {
        self.stationary_components.try_get_mut(entity)
    }

    pub fn try_get_stationary_component(&self, entity: Entity) -> Option<&StationaryComponent> {
        self.stationary_components.try_get(entity)
    }

    pub fn get_orbitable_component_mut(&mut self, entity: Entity) -> &mut OrbitableComponent {
        self.orbitable_components.get_mut(entity)
    }

    pub fn get_orbitable_component(&self, entity: Entity) -> &OrbitableComponent {
        self.orbitable_components.get(entity)
    }

    pub fn try_get_orbitable_component_mut(&mut self, entity: Entity) -> Option<&mut OrbitableComponent> {
        self.orbitable_components.try_get_mut(entity)
    }

    pub fn try_get_orbitable_component(&self, entity: Entity) -> Option<&OrbitableComponent> {
        self.orbitable_components.try_get(entity)
    }

    pub fn get_trajectory_component_mut(&mut self, entity: Entity) -> &mut TrajectoryComponent {
        self.trajectory_components.get_mut(entity)
    }

    pub fn get_trajectory_component(&self, entity: Entity) -> &TrajectoryComponent {
        self.trajectory_components.get(entity)
    }

    pub fn try_get_trajectory_component_mut(&mut self, entity: Entity) -> Option<&mut TrajectoryComponent> {
        self.trajectory_components.try_get_mut(entity)
    }

    pub fn try_get_trajectory_component(&self, entity: Entity) -> Option<&TrajectoryComponent> {
        self.trajectory_components.try_get(entity)
    }

    pub fn get_vessel_component_mut(&mut self, entity: Entity) -> &mut VesselComponent {
        self.vessel_components.get_mut(entity)
    }

    pub fn get_vessel_component(&self, entity: Entity) -> &VesselComponent {
        self.vessel_components.get(entity)
    }

    pub fn try_get_vessel_component_mut(&mut self, entity: Entity) -> Option<&mut VesselComponent> {
        self.vessel_components.try_get_mut(entity)
    }

    pub fn try_get_vessel_component(&self, entity: Entity) -> Option<&VesselComponent> {
        self.vessel_components.try_get(entity)
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