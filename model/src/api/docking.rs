use crate::{components::{path_component::{orbit::Orbit, PathComponent}, vessel_component::station::{DockingPort, DockingPortLocation, DOCKING_DISTANCE, DOCKING_SPEED}, ComponentType}, storage::entity_allocator::Entity, Model};

const EXTRA_UNDOCK_VELOCITY: f64 = 1.0;

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

    /// Finds which docking port an entity is docked to
    pub fn find_docking_port(&self, station: Entity, docked: Entity) -> Option<DockingPortLocation> {
        self.vessel_component(station)
            .as_station()
            .unwrap()
            .docking_ports()
            .iter()
            .find(|(_, docking_port)| docking_port.is_some_and(|docking_port| docking_port.docked_entity() == docked))
            .map(|entry| *entry.0)
    }

    /// Find the station to which an entity is docked (if any)
    pub fn find_station_docked_to(&self, entity: Entity) -> Option<Entity> {
        self.entities(vec![ComponentType::VesselComponent, ComponentType::PathComponent])
            .into_iter()
            .find(|&station| self.vessel_component(station).as_station().is_some() && self.find_docking_port(station, entity).is_some())
    }
    
    pub fn dock(&mut self, station: Entity, entity: Entity) {
        assert!(self.can_ever_dock_to_target(entity));
        assert!(self.can_dock(entity));
        self.path_components.remove_if_exists(entity);
        let docking_port_location = self.find_free_docking_port(station).unwrap();
        self.vessel_component_mut(station)
            .as_station_mut()
            .unwrap()
            .dock(docking_port_location, entity);
    }

    pub fn undock(&mut self, station: Entity, entity: Entity) {
        assert!(self.docked(entity));
        let docking_port_location = self.find_docking_port(station, entity).unwrap();
        self.vessel_component_mut(station)
            .as_station_mut()
            .unwrap()
            .undock(docking_port_location);
        
        let parent = self.parent(station).unwrap();
        let mass = self.vessel_component(entity).mass();
        let parent_mass = self.mass(parent);
        let position = self.position(station);
        let extra_velocity = self.velocity(station).normalize() * EXTRA_UNDOCK_VELOCITY;
        let velocity = self.velocity(station) + extra_velocity;
        let time = self.time;
        let orbit = Orbit::new(parent, mass, parent_mass, position, velocity, time);
        self.path_components.set(entity, Some(PathComponent::new_with_orbit(orbit)));
        self.recompute_trajectory(entity);
    }

    pub fn docked(&self, entity: Entity) -> bool {
        self.try_vessel_component(entity).is_some() && self.try_path_component(entity).is_none()
    }

    pub fn get_docking_port(&self, entity: Entity, location: DockingPortLocation) -> DockingPort {
        self.vessel_component(entity)
            .as_station()
            .unwrap()
            .docking_ports()
            .get(&location)
            .unwrap()
            .unwrap()
    }

    pub fn get_docking_port_mut(&mut self, entity: Entity, location: DockingPortLocation) -> &mut DockingPort {
        self.vessel_component_mut(entity)
            .as_station_mut()
            .unwrap()
            .docking_ports_mut()
            .get_mut(&location)
            .unwrap()
            .as_mut()
            .unwrap()
    }

    pub fn can_transfer_fuel_to_docked(&self, entity: Entity, location: DockingPortLocation) -> bool {
        let from = entity;
        let to = self.get_docking_port(entity, location).docked_entity();
        !self.vessel_component(from).is_fuel_empty() && !self.vessel_component(to).is_fuel_full()
    }

    pub fn can_transfer_fuel_from_docked(&self, entity: Entity, location: DockingPortLocation) -> bool {
        let to = entity;
        let from = self.get_docking_port(entity, location).docked_entity();
        !self.vessel_component(from).is_fuel_empty() && !self.vessel_component(to).is_fuel_full()
    }

    pub fn can_transfer_torpedoes_to_docked(&self, entity: Entity, location: DockingPortLocation) -> bool {
        let from = entity;
        let to = self.get_docking_port(entity, location).docked_entity();
        !self.vessel_component(from).is_torpedoes_empty() && !self.vessel_component(to).is_torpedoes_full()
    }

    pub fn can_transfer_torpedoes_from_docked(&self, entity: Entity, location: DockingPortLocation) -> bool {
        let to = entity;
        let from = self.get_docking_port(entity, location).docked_entity();
        !self.vessel_component(from).is_torpedoes_empty() && !self.vessel_component(to).is_torpedoes_full()
    }
}