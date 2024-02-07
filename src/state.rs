use std::collections::HashSet;

use crate::{storage::{entity_allocator::{EntityAllocator, Entity}, component_storage::ComponentStorage, entity_builder::EntityBuilder}, components::{name_component::NameComponent, ComponentType, mass_component::MassComponent, orbitable_component::OrbitableComponent, trajectory_component::TrajectoryComponent, stationary_component::StationaryComponent}};

#[derive(Debug)]
pub struct State {
    entity_allocator: EntityAllocator,
    mass_components: ComponentStorage<MassComponent>,
    name_components: ComponentStorage<NameComponent>,
    orbitable_components: ComponentStorage<OrbitableComponent>,
    stationary_components: ComponentStorage<StationaryComponent>,
    trajectory_components: ComponentStorage<TrajectoryComponent>,
}

impl State {
    pub fn new() -> Self {
        Self {
            entity_allocator: EntityAllocator::new(),
            mass_components: ComponentStorage::new(),
            name_components: ComponentStorage::new(),
            orbitable_components: ComponentStorage::new(),
            trajectory_components: ComponentStorage::new(),
            stationary_components: ComponentStorage::new(),
        }
    }

    pub fn mock() -> Self {
        Self {
            entity_allocator: EntityAllocator::new(),
            name_components: ComponentStorage::new(),
            mass_components: ComponentStorage::new(),
            orbitable_components: ComponentStorage::new(),
            trajectory_components: ComponentStorage::new(),
            stationary_components: ComponentStorage::new(),
        }
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
        } = entity_builder;
        let entity = self.entity_allocator.allocate();
        self.mass_components.set(entity, mass_component);
        self.name_components.set(entity, name_component);
        self.orbitable_components.set(entity, orbitable_component);
        self.stationary_components.set(entity, stationary_component);
        self.trajectory_components.set(entity, trajectory_component);
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

    pub fn get_name_component_mut(&mut self, entity: Entity) -> &mut NameComponent {
        self.name_components.get_mut(entity)
    }

    pub fn get_name_component(&self, entity: Entity) -> &NameComponent {
        self.name_components.get(entity)
    }

    pub fn get_stationary_component_mut(&mut self, entity: Entity) -> &mut StationaryComponent {
        self.stationary_components.get_mut(entity)
    }

    pub fn get_stationary_component(&self, entity: Entity) -> &StationaryComponent {
        self.stationary_components.get(entity)
    }

    pub fn get_orbitable_component_mut(&mut self, entity: Entity) -> &mut OrbitableComponent {
        self.orbitable_components.get_mut(entity)
    }

    pub fn get_orbitable_component(&self, entity: Entity) -> &OrbitableComponent {
        self.orbitable_components.get(entity)
    }

    pub fn get_trajectory_component_mut(&mut self, entity: Entity) -> &mut TrajectoryComponent {
        self.trajectory_components.get_mut(entity)
    }

    pub fn get_trajectory_component(&self, entity: Entity) -> &TrajectoryComponent {
        self.trajectory_components.get(entity)
    }
    
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{storage::entity_builder::EntityBuilder, components::{name_component::NameComponent, ComponentType}};

    use super::State;

    #[test]
    fn test_components() {
        let mut state = State::new();
        let builder1 = EntityBuilder::new().with_name_component(NameComponent::new("oh no".to_string()));
        let builder2 = EntityBuilder::new();
        let e1 = state.allocate(builder1);
        let e2 = state.allocate(builder2);

        let mut expected = HashSet::new();
        expected.insert(e1);
        expected.insert(e2);
        assert!(state.get_entities(vec![]) == expected);

        let mut expected = HashSet::new();
        expected.insert(e1);
        assert!(state.get_entities(vec![ComponentType::NameComponent]) == expected);
    }
}