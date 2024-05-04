use log::{error, trace};

use crate::components::ComponentType;

use super::{storage::entity_allocator::Entity, Model};

impl Model {
    #[allow(unused)]
    pub fn entity_by_name(&self, name: &str) -> Entity {
        for entity in self.entities(vec![ComponentType::NameComponent]) {
            if self.name_component(entity).name() == name {
                return entity;
            }
        }
        error!("No entity '{}' found", name);
        Entity::mock()
    }

    #[allow(unused)]
    pub fn log_components(&self, entity: Entity) {
        if let Some(component) = self.try_name_component(entity) {
            trace!("{component:?}");
        }
        if let Some(component) = self.try_orbitable_component(entity) {
            trace!("{component:?}");
        }
        if let Some(component) = self.try_path_component(entity) {
            trace!("{component:?}");
        }
        if let Some(component) = self.try_vessel_component(entity) {
            trace!("{component:?}");
        }
    }
}