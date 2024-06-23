use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use super::{faction::Faction, timeline::Timeline};

pub const DOCKING_DISTANCE: f64 = 1.0e2;
pub const DOCKING_SPEED: f64 = 10.0;

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

    pub fn default_docking_ports(&self) -> HashMap<DockingPortLocation, Option<Entity>> {
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
    docking_ports: HashMap<DockingPortLocation, Option<Entity>>,
}

impl Station {
    pub fn new(class: StationClass, faction: Faction) -> Self {
        let target = None;
        let timeline = Timeline::default();
        let docking_ports = class.default_docking_ports();
        Self { class, faction, target, timeline, docking_ports }
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
        0.0
    }

    pub fn max_fuel_kg(&self) -> f64 {
        0.0
    }

    pub fn fuel_litres(&self) -> f64 {
        0.0
    }

    pub fn fuel_kg(&self) -> f64 {
        0.0
    }

    pub fn specific_impulse(&self) -> Option<f64> {
        None
    }

    pub fn fuel_kg_per_second(&self) -> Option<f64> {
        None
    }
    
    pub fn docking_ports(&self) -> &HashMap<DockingPortLocation, Option<Entity>> {
        &self.docking_ports
    }

    pub fn dock(&mut self, location: DockingPortLocation, entity: Entity) {
        self.docking_ports.insert(location, Some(entity));
    }

    pub fn undock(&mut self, location: DockingPortLocation) {
        self.docking_ports.insert(location, None);
        //TODO create path component
    }
}