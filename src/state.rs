use std::collections::HashSet;

use crate::{storage::{entity_allocator::{EntityAllocator, Entity}, component_storage::ComponentStorage, entity_builder::EntityBuilder}, components::{name_component::NameComponent, ComponentType, physics_component::PhysicsComponent}};

pub struct State {
    entity_allocator: EntityAllocator,
    name_components: ComponentStorage<NameComponent>,
    physics_components: ComponentStorage<PhysicsComponent>,
}

impl State {
    pub fn new() -> Self {
        Self {
            entity_allocator: EntityAllocator::new(),
            name_components: ComponentStorage::new(),
            physics_components: ComponentStorage::new(),
        }
    }

    pub fn get_entities(&self, mut component_types: Vec<ComponentType>) -> HashSet<Entity> {
        let mut entities = self.entity_allocator.get_entities().clone();
        while let Some(component_type) = component_types.pop() {
            let other_entities = match component_type {
                ComponentType::NameComponent => self.name_components.get_entities(),
                ComponentType::TrajectoryComponent => self.physics_components.get_entities(),
            };
            entities.retain(|entity| other_entities.contains(entity));
        }
        entities
    }

    pub fn allocate(&mut self, entity_builder: EntityBuilder) -> Entity {
        let EntityBuilder {
            name_component,
            physics_component,
        } = entity_builder;
        let entity = self.entity_allocator.allocate();
        self.name_components.set(entity, name_component);
        self.physics_components.set(entity, physics_component);
        entity
    }

    pub fn deallocate(&mut self, entity: Entity) {
        self.entity_allocator.deallocate(entity);
        self.name_components.remove_if_exists(entity);
    }

    pub fn get_name_component_mut(&mut self, entity: Entity) -> &mut NameComponent {
        self.name_components.get_mut(entity)
    }

    pub fn get_name_component(&self, entity: Entity) -> &NameComponent {
        self.name_components.get(entity)
    }

    pub fn get_physics_component_mut(&mut self, entity: Entity) -> &mut PhysicsComponent {
        self.physics_components.get_mut(entity)
    }

    pub fn get_physics_component(&self, entity: Entity) -> &PhysicsComponent {
        self.physics_components.get(entity)
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