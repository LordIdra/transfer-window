use crate::{components::{name_component::NameComponent, orbitable_component::OrbitableComponent, path_component::PathComponent, vessel_component::VesselComponent}, storage::entity_allocator::Entity};

use super::Model;

impl Model {
    pub fn name_component(&self, entity: Entity) -> &NameComponent {
        self.name_components.get(entity)
    }

    pub fn try_name_component(&self, entity: Entity) -> Option<&NameComponent> {
        self.name_components.try_get(entity)
    }

    pub fn has_name_component(&self, entity: Entity) -> bool {
        self.try_name_component(entity).is_some()
    }

    pub fn name_component_mut(&mut self, entity: Entity) -> &mut NameComponent {
        self.name_components.get_mut(entity)
    }

    pub fn try_name_component_mut(&mut self, entity: Entity) -> Option<&mut NameComponent> {
        self.name_components.try_get_mut(entity)
    }

    pub fn orbitable_component(&self, entity: Entity) -> &OrbitableComponent {
        self.orbitable_components.get(entity)
    }

    pub fn try_orbitable_component(&self, entity: Entity) -> Option<&OrbitableComponent> {
        self.orbitable_components.try_get(entity)
    }

    pub fn has_orbitable_component(&self, entity: Entity) -> bool {
        self.try_orbitable_component(entity).is_some()
    }

    pub fn orbitable_component_mut(&mut self, entity: Entity) -> &mut OrbitableComponent {
        self.orbitable_components.get_mut(entity)
    }

    pub fn try_orbitable_component_mut(&mut self, entity: Entity) -> Option<&mut OrbitableComponent> {
        self.orbitable_components.try_get_mut(entity)
    }

    pub fn path_component(&self, entity: Entity) -> &PathComponent {
        self.path_components.get(entity)
    }

    pub fn try_path_component(&self, entity: Entity) -> Option<&PathComponent> {
        self.path_components.try_get(entity)
    }

    pub fn has_path_component(&self, entity: Entity) -> bool {
        self.try_path_component(entity).is_some()
    }

    pub fn path_component_mut(&mut self, entity: Entity) -> &mut PathComponent {
        self.path_components.get_mut(entity)
    }

    pub fn try_path_component_mut(&mut self, entity: Entity) -> Option<&mut PathComponent> {
        self.path_components.try_get_mut(entity)
    }

    pub fn vessel_component(&self, entity: Entity) -> &VesselComponent {
        self.vessel_components.get(entity)
    }

    pub fn try_vessel_component(&self, entity: Entity) -> Option<&VesselComponent> {
        self.vessel_components.try_get(entity)
    }

    pub fn has_vessel_component(&self, entity: Entity) -> bool {
        self.try_vessel_component(entity).is_some()
    }

    pub fn vessel_component_mut(&mut self, entity: Entity) -> &mut VesselComponent {
        self.vessel_components.get_mut(entity)
    }

    pub fn try_vessel_component_mut(&mut self, entity: Entity) -> Option<&mut VesselComponent> {
        self.vessel_components.try_get_mut(entity)
    }

}
