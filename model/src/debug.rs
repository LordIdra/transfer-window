use log::trace;

use crate::{components::ComponentType, model::Model};

use super::storage::entity_allocator::Entity;

impl Model {
    #[allow(unused)]
    pub fn entity_by_name(&self, name: &str) -> Option<Entity> {
        self.entities(vec![ComponentType::NameComponent])
            .into_iter()
            .find(|&entity| self.name_component(entity).name() == name)
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
