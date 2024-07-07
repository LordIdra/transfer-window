use serde::{Deserialize, Serialize};

pub const FUEL_DENSITY: f64 = 1.0; // both RP-1 and LOX are very roughly 1.0kg/L

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum FuelTankType {
    Torpedo,
    Hub,
    FuelTank1,
    FuelTank2,
    FuelTank3,
    FuelTank4,
}

impl FuelTankType {
    pub fn ship_types() -> [Self; 4] {
        [
            FuelTankType::FuelTank1, 
            FuelTankType::FuelTank2,
            FuelTankType::FuelTank3,
            FuelTankType::FuelTank4,
        ]
    }

    pub fn capacity_litres(&self) -> f64 {
        match self {
            FuelTankType::Torpedo => 1000.0,
            FuelTankType::Hub => 25000.0,
            FuelTankType::FuelTank1 => 8000.0,
            FuelTankType::FuelTank2 => 10000.0,
            FuelTankType::FuelTank3 => 14000.0,
            FuelTankType::FuelTank4 => 20000.0,
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

impl FuelTank {
    pub fn new(type_: FuelTankType) -> Self {
        let remaining_litres = type_.capacity_litres() / 2.0;
        FuelTank { type_, remaining_litres }
    }

    pub fn type_(&self) -> FuelTankType {
        self.type_
    }

    pub fn capacity_litres(&self) -> f64 {
        self.type_.capacity_litres()
    }

    pub fn capacity_kg(&self) -> f64 {
        self.type_.capacity_kg()
    }

    pub fn fuel_litres(&self) -> f64 {
        self.remaining_litres
    }

    pub fn fuel_kg(&self) -> f64 {
        self.remaining_litres * FUEL_DENSITY
    }

    pub fn set_fuel_kg(&mut self, new_fuel_kg: f64) {
        self.remaining_litres = new_fuel_kg / FUEL_DENSITY;
    }
}