use serde::{Deserialize, Serialize};

use super::{System, SystemType};

pub const FUEL_DENSITY: f64 = 1.0; // both RP-1 and LOX are very roughly 1.0kg/L

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum FuelTankType {
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

    pub fn get_capacity_litres(&self) -> f64 {
        match self {
            FuelTankType::Small => 10000.0,
            FuelTankType::Medium => 15000.0,
            FuelTankType::Large => 20000.0,
        }
    }

    pub fn get_capacity_kg(&self) -> f64 {
        self.get_capacity_litres() * FUEL_DENSITY
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuelTank {
    type_: FuelTankType,
    remaining_litres: f64,
}

impl System for FuelTank {
    type Type = FuelTankType;
    
    fn get_type(&self) -> &Self::Type {
        &self.type_
    }
}

impl FuelTank {
    pub fn new(type_: FuelTankType) -> Self {
        FuelTank { type_, remaining_litres: type_.get_capacity_litres() }
    }

    pub fn get_remaining_litres(&self) -> f64 {
        self.remaining_litres
    }

    pub fn get_remaining_kg(&self) -> f64 {
        self.remaining_litres * FUEL_DENSITY
    }

    pub fn set_remaining(&mut self, to_deplete_kg: f64) {
        self.remaining_litres = to_deplete_kg / FUEL_DENSITY;
    }
}