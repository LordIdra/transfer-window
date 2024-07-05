use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

pub const DOCKING_DISTANCE: f64 = 1.0e2;
pub const DOCKING_SPEED: f64 = 10.0;
pub const FUEL_TRANSFER_RATE: f64 = 0.1;
pub const TORPEDO_TRANSFER_TIME: f64 = 1800.0;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DockingPortLocation {
    North,
    East,
    South,
    West,
}

impl DockingPortLocation {
    pub fn name(&self) -> &'static str {
        match self {
            DockingPortLocation::North => "North",
            DockingPortLocation::East => "East",
            DockingPortLocation::South => "South",
            DockingPortLocation::West => "West",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DockingType {
    Quadruple
}

impl DockingType {
    pub fn docking_port_locations(&self) -> Vec<DockingPortLocation> {
        match self {
            DockingType::Quadruple => vec![DockingPortLocation::North, DockingPortLocation::East, DockingPortLocation::South, DockingPortLocation::West],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ResourceTransferDirection {
    ToDocked,
    FromDocked,
}

impl ResourceTransferDirection {
    pub fn is_to_docked(&self) -> bool {
        matches!(self, Self::ToDocked)
    }

    pub fn is_from_docked(&self) -> bool {
        matches!(self, Self::FromDocked)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContinuousResourceTransfer {
    direction: ResourceTransferDirection,
    rate: f64,
}

impl ContinuousResourceTransfer {
    pub fn new(direction: ResourceTransferDirection) -> Self {
        Self { direction, rate: FUEL_TRANSFER_RATE }
    }

    pub fn direction(&self) -> ResourceTransferDirection {
        self.direction
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscreteResourceTransfer {
    direction: ResourceTransferDirection,
    interval: f64,
    time_to_next: f64,
}

impl DiscreteResourceTransfer {
    pub fn new(direction: ResourceTransferDirection) -> Self {
        Self { direction, interval: TORPEDO_TRANSFER_TIME, time_to_next: TORPEDO_TRANSFER_TIME }
    }

    pub fn direction(&self) -> ResourceTransferDirection {
        self.direction
    }

    pub fn interval(&self) -> f64 {
        self.interval
    }

    pub fn time_to_next(&self) -> f64 {
        self.time_to_next
    }

    pub fn step(&self, dt: f64) -> Self {
        let direction = self.direction;
        let interval = self.interval;
        let time_to_next = self.time_to_next - dt;
        Self { direction, interval, time_to_next }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DockedVessel {
    entity: Entity,
    fuel_transfer: Option<ContinuousResourceTransfer>,
    torpedo_transfer: Option<DiscreteResourceTransfer>,
}

impl DockedVessel {
    pub fn new(entity: Entity) -> Self {
        let fuel_transfer = None;
        let torpedo_transfer = None;
        Self { entity, fuel_transfer, torpedo_transfer }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn fuel_transfer(&self) -> Option<&ContinuousResourceTransfer> {
        self.fuel_transfer.as_ref()
    }
    
    pub fn torpedo_transfer(&self) -> Option<&DiscreteResourceTransfer> {
        self.torpedo_transfer.as_ref()
    }
    
    pub fn start_fuel_transfer(&mut self, direction: ResourceTransferDirection) {
        self.fuel_transfer = Some(ContinuousResourceTransfer::new(direction));
    }

    pub fn stop_fuel_transfer(&mut self) {
        self.fuel_transfer = None;
    }

    pub fn start_torpedo_transfer(&mut self, direction: ResourceTransferDirection) {
        self.torpedo_transfer = Some(DiscreteResourceTransfer::new(direction));
    }

    pub fn step_torpedo_transfer(&mut self, dt: f64) {
        self.torpedo_transfer = Some(self.torpedo_transfer.as_mut().unwrap().step(dt));
    }

    pub fn stop_torpedo_transfer(&mut self) {
        self.torpedo_transfer = None;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DockingPort {
    docked_vessel: Option<DockedVessel>,
}

impl Default for DockingPort {
    fn default() -> Self {
        let docked_vessel = None;
        Self { docked_vessel }
    }
}

impl DockingPort {
    pub fn has_docked_vessel(&self) -> bool {
        self.docked_vessel.is_some()
    }

    pub fn docked_vessel(&self) -> &DockedVessel {
        self.docked_vessel.as_ref().expect("Attempt to get vessel from docking port without docked vessel")
    }

    pub fn docked_vessel_mut(&mut self) -> &mut DockedVessel {
        self.docked_vessel.as_mut().expect("Attempt to get vessel from docking port without docked vessel")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Docking {
    type_: DockingType,
    docking_ports: HashMap<DockingPortLocation, DockingPort>,
}

impl Docking {
    pub fn new(type_: DockingType) -> Self {
        let docking_ports = type_.docking_port_locations()
            .into_iter()
            .map(|location| (location, DockingPort::default()))
            .collect();
        Self { type_, docking_ports }
    }

    pub fn type_(&self) -> DockingType {
        self.type_
    }

    pub fn docking_ports(&self) -> &HashMap<DockingPortLocation, DockingPort> {
        &self.docking_ports
    }

    pub fn docking_ports_mut(&mut self) -> &mut HashMap<DockingPortLocation, DockingPort> {
        &mut self.docking_ports
    }

    pub fn docking_port(&self, location: DockingPortLocation) -> &DockingPort {
        self.docking_ports.get(&location).expect("Attempt to get nonexistant docking port")
    }

    pub(crate) fn docking_port_mut(&mut self, location: DockingPortLocation) -> &mut DockingPort {
        self.docking_ports.get_mut(&location).expect("Attempt to get nonexistant docking port")
    }

    pub fn dock(&mut self, location: DockingPortLocation, entity: Entity) {
        self.docking_port_mut(location).docked_vessel = Some(DockedVessel::new(entity));
    }

    pub fn undock(&mut self, location: DockingPortLocation) {
        self.docking_port_mut(location).docked_vessel = None;
    }
}