use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum FuelTankType {
    Small,
    Medium,
    Large
}

impl FuelTankType {
    pub fn get_capacity(&self) -> f64 {
        match self {
            FuelTankType::Small => 10000.0,
            FuelTankType::Medium => 15000.0,
            FuelTankType::Large => 20000.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuelTank {
    type_: FuelTankType,
    remaining: f64,
}

impl FuelTank {
    pub fn new(type_: FuelTankType) -> Self {
        FuelTank { type_, remaining: type_.get_capacity() }
    }
    
    pub fn type_(&self) -> &FuelTankType {
        &self.type_
    }
}