use crate::{components::vessel_component::station::{DockingPortLocation, DOCKING_DISTANCE, DOCKING_SPEED}, storage::entity_allocator::Entity, Model};

impl Model {
    pub fn can_ever_dock_to_target(&self, entity: Entity) -> bool {
        let target = self.target(entity).unwrap();
        self.vessel_component(entity).can_dock() && self.vessel_component(target).as_station().is_some()
    }

    pub fn can_dock(&self, entity: Entity) -> bool {
        let target = self.target(entity).unwrap();
        self.distance(entity, target) < DOCKING_DISTANCE && self.relative_speed(entity, target) < DOCKING_SPEED
    }

    pub fn find_free_docking_port(&self, entity: Entity) -> Option<DockingPortLocation> {
        self.vessel_component(entity)
            .as_station()
            .unwrap()
            .docking_ports()
            .iter()
            .find(|(_, docked_entity)| docked_entity.is_none())
            .map(|entry| *entry.0)
    }
    
    pub fn dock(&mut self, entity: Entity) {
        assert!(self.can_ever_dock_to_target(entity));
        assert!(self.can_dock(entity));
        let target = self.target(entity).unwrap();
        self.path_components.remove_if_exists(entity);
        let docking_port_location = self.find_free_docking_port(target).unwrap();
        self.vessel_component_mut(target)
            .as_station_mut()
            .unwrap()
            .dock(docking_port_location, entity);
    }

    pub fn docked(&self, entity: Entity) -> bool {
        self.try_vessel_component(entity).is_some() && self.try_path_component(entity).is_none()
    }
}