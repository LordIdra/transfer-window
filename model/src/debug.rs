use log::{error, trace};

use crate::components::ComponentType;

use super::{storage::entity_allocator::Entity, Model};

#[allow(unused)]
pub fn get_entity_by_name(model: &Model, name: &str) -> Entity {
    for entity in model.get_entities(vec![ComponentType::NameComponent]) {
        if model.get_name_component(entity).get_name() == name {
            return entity;
        }
    }
    error!("No entity '{}' found", name);
    Entity::mock()
}

#[allow(unused)]
pub fn log_components(model: &Model, entity: Entity) {
    if let Some(component) = model.try_get_name_component(entity) {
        trace!("{component:?}");
    }
    if let Some(component) = model.try_get_orbitable_component(entity) {
        trace!("{component:?}");
    }
    if let Some(component) = model.try_get_stationary_component(entity) {
        trace!("{component:?}");
    }
    if let Some(component) = model.try_get_trajectory_component(entity) {
        trace!("{component:?}");
    }
    if let Some(component) = model.try_get_vessel_component(entity) {
        trace!("{component:?}");
    }
}