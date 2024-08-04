use serde::{Deserialize, Serialize};

use super::{docking::DockingType, engine::EngineType, faction::Faction, fuel_tank::FuelTankType, torpedo_storage::TorpedoStorageType, VesselComponent};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum VesselClass {
    Torpedo,
    Hub,
    Scout1,
    Scout2,
    Frigate1,
    Frigate2,
}

impl VesselClass {
    pub fn name(&self) -> &'static str {
        match self {
            VesselClass::Torpedo => "Torpedo",
            VesselClass::Hub => "Hub",
            VesselClass::Scout1 => "Scout I",
            VesselClass::Scout2 => "Scout II",
            VesselClass::Frigate1 => "Frigate I",
            VesselClass::Frigate2 => "Frigate II",
        }
    }

    pub fn build(&self, faction: Faction) -> VesselComponent {
        match self {
            VesselClass::Torpedo => VesselComponent::new(*self, faction)
                .with_fuel_tank(FuelTankType::Torpedo)
                .with_engine(EngineType::Torpedo)
                .with_ghost(),
            VesselClass::Hub => VesselComponent::new(*self, faction)
                .with_fuel_tank(FuelTankType::Hub)
                .with_torpedo_storage(TorpedoStorageType::Hub)
                .with_docking(DockingType::Quadruple),
            VesselClass::Scout1 | VesselClass::Scout2 | VesselClass::Frigate1 | VesselClass::Frigate2 => VesselComponent::new(*self, faction),
        }
    }

    pub fn mass(&self) -> f64 {
        match self {
            VesselClass::Torpedo => 2.0e3,
            VesselClass::Hub => 380.0e3,
            VesselClass::Scout1 => 10.0e3,
            VesselClass::Scout2 => 12.0e3,
            VesselClass::Frigate1 => 30.0e3,
            VesselClass::Frigate2 => 36.0e3,
        }
    }

    pub fn is_torpedo(&self) -> bool {
        matches!(self, VesselClass::Torpedo)
    }

    pub fn dockable(&self) -> bool {
        match self {
            VesselClass::Torpedo | VesselClass::Hub => false,
            VesselClass::Scout1 | VesselClass::Scout2 | VesselClass::Frigate1 | VesselClass::Frigate2 => true,
        }
    }

    pub fn editable(&self) -> bool {
        match self {
            VesselClass::Torpedo | VesselClass::Hub => false,
            VesselClass::Scout1 | VesselClass::Scout2 | VesselClass::Frigate1 | VesselClass::Frigate2 => true,
        }
    }
}
