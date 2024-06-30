use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use super::{faction::Faction, ship::ship_slot::fuel_tank::FUEL_DENSITY, timeline::Timeline};

pub const DOCKING_DISTANCE: f64 = 1.0e2;
pub const DOCKING_SPEED: f64 = 10.0;
pub const FUEL_TRANSFER_RATE: f64 = 0.1;
pub const TORPEDO_TRANSFER_TIME: f64 = 1800.0;

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct DockingPort {
    docked_entity: Entity,
    fuel_transfer: Option<ContinuousResourceTransfer>,
    torpedo_transfer: Option<DiscreteResourceTransfer>,
}

impl DockingPort {
    pub fn new(docked_entity: Entity) -> Self {
        let fuel_transfer = None;
        let torpedo_transfer = None;
        Self { docked_entity, fuel_transfer, torpedo_transfer }
    }

    pub fn docked_entity(&self) -> Entity {
        self.docked_entity
    }

    pub fn fuel_transfer(&self) -> Option<ContinuousResourceTransfer> {
        self.fuel_transfer
    }
    
    pub fn torpedo_transfer(&self) -> Option<DiscreteResourceTransfer> {
        self.torpedo_transfer
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
        self.torpedo_transfer = Some(self.torpedo_transfer.unwrap().step(dt));
    }

    pub fn stop_torpedo_transfer(&mut self) {
        self.torpedo_transfer = None;
    }
}

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum StationClass {
    Hub,
    Outpost,
}

impl StationClass {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Hub => "Hub",
            Self::Outpost => "Outpost",
        }
    }

    pub fn mass(&self) -> f64 {
        match self {
            Self::Hub => 60.0e4,
            Self::Outpost => 12.0e4,
        }
    }

    pub fn max_fuel_litres(&self) -> f64 {
        match self {
            Self::Hub => 13.0e3,
            Self::Outpost => 8.0e3,
        }
    }

    pub fn max_torpedoes(&self) -> usize {
        match self {
            StationClass::Hub => 8,
            StationClass::Outpost => 4,
        }
    }

    pub fn default_docking_ports(&self) -> HashMap<DockingPortLocation, Option<DockingPort>> {
        match self {
            StationClass::Hub => [
                (DockingPortLocation::North, None),
                (DockingPortLocation::East, None),
                (DockingPortLocation::South, None),
                (DockingPortLocation::West, None),
            ].iter().copied().collect(),
            StationClass::Outpost => [
                (DockingPortLocation::North, None),
                (DockingPortLocation::South, None),
            ].iter().copied().collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Station {
    class: StationClass,
    faction: Faction,
    target: Option<Entity>,
    timeline: Timeline,
    docking_ports: HashMap<DockingPortLocation, Option<DockingPort>>,
    fuel_litres: f64,
    torpedoes: usize,
}

impl Station {
    pub fn new(class: StationClass, faction: Faction) -> Self {
        let target = None;
        let timeline = Timeline::default();
        let docking_ports = class.default_docking_ports();
        let fuel_litres = class.max_fuel_litres() / 2.0;
        let torpedoes = class.max_torpedoes() / 2;
        Self { class, faction, target, timeline, docking_ports, fuel_litres, torpedoes }
    }

    pub fn class(&self) -> StationClass {
        self.class
    }

    pub fn faction(&self) -> Faction {
        self.faction
    }

    pub fn target(&self) -> Option<Entity> {
        self.target
    }

    pub fn set_target(&mut self, target: Option<Entity>) {
        self.target = target;
    }

    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }

    pub fn timeline_mut(&mut self) -> &mut Timeline {
        &mut self.timeline
    }

    pub fn dry_mass(&self) -> f64 {
        self.class.mass()
    }

    pub fn wet_mass(&self) -> f64 {
        self.class.mass()
    }

    pub fn mass(&self) -> f64 {
        self.class.mass()
    }

    pub fn max_fuel_litres(&self) -> f64 {
        self.class.max_fuel_litres()
    }

    pub fn max_fuel_kg(&self) -> f64 {
        self.max_fuel_litres() * FUEL_DENSITY
    }

    pub fn fuel_litres(&self) -> f64 {
        self.fuel_litres
    }

    pub fn fuel_kg(&self) -> f64 {
        self.fuel_litres() * FUEL_DENSITY
    }

    pub fn specific_impulse(&self) -> Option<f64> {
        None
    }

    pub fn fuel_kg_per_second(&self) -> Option<f64> {
        None
    }

    pub fn set_fuel_kg(&mut self, new_fuel_kg: f64) {
        self.fuel_litres = new_fuel_kg / FUEL_DENSITY;
    }

    pub fn max_torpedoes(&self) -> usize {
        self.class.max_torpedoes()
    }

    pub fn torpedoes(&self) -> usize {
        self.torpedoes
    }

    pub fn decrement_torpedoes(&mut self) {
        self.torpedoes -= 1;
    }

    pub fn increment_torpedoes(&mut self) {
        self.torpedoes += 1;
    }
    
    pub fn docking_ports(&self) -> &HashMap<DockingPortLocation, Option<DockingPort>> {
        &self.docking_ports
    }

    pub fn docking_ports_mut(&mut self) -> &mut HashMap<DockingPortLocation, Option<DockingPort>> {
        &mut self.docking_ports
    }

    pub fn dock(&mut self, location: DockingPortLocation, entity: Entity) {
        self.docking_ports.insert(location, Some(DockingPort::new(entity)));
    }

    pub fn undock(&mut self, location: DockingPortLocation) {
        self.docking_ports.insert(location, None);
    }
}