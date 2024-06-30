use serde::{Deserialize, Serialize};

use super::{docking::DockingType, engine::EngineType, faction::Faction, fuel_tank::FuelTankType, VesselComponent};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum VesselClass {
    Torpedo,
    Hub,
    Scout,
    Frigate,
}

impl VesselClass {
    pub fn name(&self) -> &'static str {
        match self {
            VesselClass::Torpedo => "Torpedo",
            VesselClass::Hub => "Hub",
            VesselClass::Scout => "Scout",
            VesselClass::Frigate => "Frigate",
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
                .with_docking(DockingType::Quadruple),
            VesselClass::Scout | VesselClass::Frigate => VesselComponent::new(*self, faction),
        }
    }

    pub fn mass(&self) -> f64 {
        match self {
            VesselClass::Torpedo => 2.0e3,
            VesselClass::Hub => 160.0e3,
            VesselClass::Scout => 10.0e3,
            VesselClass::Frigate => 25.0e3,
        }
    }

    pub fn is_torpedo(&self) -> bool {
        matches!(self, VesselClass::Torpedo)
    }

    pub fn can_dock(&self) -> bool {
        match self {
            VesselClass::Torpedo | VesselClass::Hub => false,
            VesselClass::Scout | VesselClass::Frigate => true,
        }
    }

    pub fn editable(&self) -> bool {
        match self {
            VesselClass::Torpedo | VesselClass::Hub => false,
            VesselClass::Scout | VesselClass::Frigate => true,
        }
    }
}