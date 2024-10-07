use crate::{components::{path_component::{orbit::builder::OrbitBuilder, PathComponent}, vessel_component::docking::{DockingPort, DockingPortLocation, DOCKING_DISTANCE, DOCKING_SPEED}, ComponentType}, storage::entity_allocator::Entity};

use super::{state_query::StateQuery, Model};

const EXTRA_UNDOCK_VELOCITY: f64 = 1.0;

impl Model {
    pub fn can_ever_dock_to_target(&self, entity: Entity) -> bool {
        let target = self.target(entity).unwrap();
        self.vessel_component(entity).can_dock() && self.vessel_component(target).has_docking()
    }

    pub fn can_dock(&self, entity: Entity) -> bool {
        let target = self.target(entity).unwrap();
        self.distance(entity, target) < DOCKING_DISTANCE 
            && self.relative_speed(entity, target) < DOCKING_SPEED
            && self.find_free_docking_port(target).is_some()
    }

    pub fn find_free_docking_port(&self, entity: Entity) -> Option<DockingPortLocation> {
        self.vessel_component(entity)
            .docking_ports()
            .expect("Attempt to find free docking ports on non-station entity")
            .iter()
            .find(|(_, docking_port)| !docking_port.has_docked_vessel())
            .map(|entry| *entry.0)
    }

    /// Finds which docking port an entity is docked to
    pub fn find_docking_port(&self, station: Entity, docked: Entity) -> Option<DockingPortLocation> {
        self.vessel_component(station)
            .docking_ports()
            .expect("Attempt to find docking port on non-station entity")
            .iter()
            .filter(|(_, docking_port)| docking_port.has_docked_vessel())
            .find(|(_, docking_port)| docking_port.docked_vessel().entity() == docked)
            .map(|entry| *entry.0)
    }

    /// Find the station to which an entity is docked (if any)
    pub fn find_station_docked_to(&self, entity: Entity) -> Option<Entity> {
        self.entities(vec![ComponentType::VesselComponent, ComponentType::PathComponent])
            .into_iter()
            .find(|&station| self.vessel_component(station).has_docking() && self.find_docking_port(station, entity).is_some())
    }
    
    pub fn dock(&mut self, station: Entity, entity: Entity) {
        assert!(self.can_ever_dock_to_target(entity));
        assert!(self.can_dock(entity));
        self.path_components.remove_if_exists(entity);
        let docking_port_location = self.find_free_docking_port(station).unwrap();
        self.vessel_component_mut(station).dock(docking_port_location, entity);
    }

    pub fn undock(&mut self, station: Entity, entity: Entity) {
        assert!(self.docked(entity));
        let docking_port_location = self.find_docking_port(station, entity).unwrap();
        self.vessel_component_mut(station).undock(docking_port_location);

        let parent = self.parent(station).unwrap();
        let extra_velocity = self.velocity(station).normalize() * EXTRA_UNDOCK_VELOCITY;
        let velocity = self.velocity(station) + extra_velocity;
        let orbit = OrbitBuilder {
            parent,
            mass: self.vessel_component(entity).mass(),
            parent_mass: self.mass(parent),
            rotation: f64::atan2(velocity.y, velocity.x),
            position: self.position(station),
            velocity,
            time: self.time,
        }.build();

        self.path_components.set(entity, Some(PathComponent::new_with_orbit(orbit)));
        self.recompute_trajectory(entity);
    }

    pub fn docked(&self, entity: Entity) -> bool {
        self.try_vessel_component(entity).is_some() && self.try_path_component(entity).is_none()
    }

    pub fn docking_port(&self, entity: Entity, location: DockingPortLocation) -> &DockingPort {
        self.vessel_component(entity)
            .docking_ports()
            .expect("Attempt to get docking port of entity without docking ports")
            .get(&location)
            .unwrap()
    }

    pub fn docking_port_mut(&mut self, entity: Entity, location: DockingPortLocation) -> &mut DockingPort {
        self.vessel_component_mut(entity)
            .docking_ports_mut()
            .expect("Attempt to get docking port of entity without docking ports")
            .get_mut(&location)
            .unwrap()
    }

    pub fn docked_entity(&self, entity: Entity, location: DockingPortLocation) -> Entity {
        self.docking_port(entity, location).docked_vessel().entity()
    }

    pub fn can_transfer_fuel_to_docked(&self, entity: Entity, location: DockingPortLocation) -> bool {
        let from = entity;
        let to = self.docked_entity(entity, location);
        !self.vessel_component(from).is_fuel_empty() && !self.vessel_component(to).is_fuel_full()
    }

    pub fn can_transfer_fuel_from_docked(&self, entity: Entity, location: DockingPortLocation) -> bool {
        let to = entity;
        let from = self.docked_entity(entity, location);
        !self.vessel_component(from).is_fuel_empty() && !self.vessel_component(to).is_fuel_full()
    }

    pub fn can_transfer_torpedoes_to_docked(&self, entity: Entity, location: DockingPortLocation) -> bool {
        let from = entity;
        let to = self.docked_entity(entity, location);
        !self.vessel_component(from).is_torpedoes_empty() && !self.vessel_component(to).is_torpedoes_full()
    }

    pub fn can_transfer_torpedoes_from_docked(&self, entity: Entity, location: DockingPortLocation) -> bool {
        let to = entity;
        let from = self.docked_entity(entity, location);
        !self.vessel_component(from).is_torpedoes_empty() && !self.vessel_component(to).is_torpedoes_full()
    }
}
