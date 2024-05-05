use serde::{Deserialize, Serialize};

use super::{System, SystemType};

pub const FUEL_DENSITY: f64 = 1.0; // both RP-1 and LOX are very roughly 1.0kg/L

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum FuelTankType {
    Torpedo,
    Small,
    Medium,
    Large
}

impl SystemType for FuelTankType {}

impl FuelTankType {
    pub const TYPES: [FuelTankType; 3] = [
        FuelTankType::Small, 
        FuelTankType::Medium,
        FuelTankType::Large,
    ];

    pub fn capacity_litres(&self) -> f64 {
        match self {
            FuelTankType::Torpedo => 2000.0,
            FuelTankType::Small => 10000.0,
            FuelTankType::Medium => 15000.0,
            FuelTankType::Large => 20000.0,
        }
    }

    pub fn capacity_kg(&self) -> f64 {
        self.capacity_litres() * FUEL_DENSITY
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuelTank {
    type_: FuelTankType,
    remaining_litres: f64,
}

impl System for FuelTank {
    type Type = FuelTankType;
    
    fn type_(&self) -> &Self::Type {
        &self.type_
    }
}

impl FuelTank {
    pub fn new(type_: FuelTankType) -> Self {
        FuelTank { type_, remaining_litres: type_.capacity_litres() }
    }

    pub fn remaining_litres(&self) -> f64 {
        self.remaining_litres
    }

    pub fn remaining_kg(&self) -> f64 {
        self.remaining_litres * FUEL_DENSITY
    }

    pub fn set_remaining(&mut self, to_deplete_kg: f64) {
        self.remaining_litres = to_deplete_kg / FUEL_DENSITY;
    }
}