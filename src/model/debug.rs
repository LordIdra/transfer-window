use crate::model::components::ComponentType;

use super::{storage::entity_allocator::Entity, Model};

pub fn get_entity_by_name(model: &Model, name: &str) -> Entity {
    for entity in model.get_entities(vec![ComponentType::NameComponent]) {
        if model.get_name_component(entity).get_name() == name {
            return entity;
        }
    }
    panic!("No entity '{}' found", name);
}