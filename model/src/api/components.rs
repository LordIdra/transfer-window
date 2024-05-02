use crate::components::vessel_component::VesselComponent;
use crate::Model;

use crate::{components::{name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::TrajectoryComponent}, storage::entity_allocator::Entity};

impl Model {
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