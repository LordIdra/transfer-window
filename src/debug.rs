use crate::{components::ComponentType, state::State, storage::entity_allocator::Entity};

pub fn get_entity_by_name(state: &State, name: &str) -> Entity {
    for entity in state.get_entities(vec![ComponentType::NameComponent]) {
        if state.get_name_component(entity).get_name() == name {
            return entity;
        }
    }
    panic!("No entity '{}' found", name);
}