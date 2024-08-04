use serde::{Deserialize, Serialize};

pub const FUEL_DENSITY_KG_PER_LITRE: f64 = 1.0; // both RP-1 and LOX are very roughly 1.0kg/L

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum FuelTankType {
    Torpedo,
    Hub,
    Tank1,
    Tank2,
    Tank3,
    Tank4,
}

impl FuelTankType {
    pub fn mass(&self) -> f64 {
        match self {
            FuelTankType::Torpedo => 0.5e3,
            FuelTankType::Hub => 2.0e3,
            FuelTankType::Tank1 => 0.7e3,
            FuelTankType::Tank2 => 1.0e3,
            FuelTankType::Tank3 => 1.2e3,
            FuelTankType::Tank4 => 1.3e3,
        }
    }

    pub fn ship_types() -> [Self; 4] {
        [
            FuelTankType::Tank1, 
            FuelTankType::Tank2,
            FuelTankType::Tank3,
            FuelTankType::Tank4,
        ]
    }

    pub fn capacity_litres(&self) -> f64 {
        match self {
            FuelTankType::Torpedo => 10_000.0,
            FuelTankType::Hub => 140_000.0,
            FuelTankType::Tank1 => 30_000.0,
            FuelTankType::Tank2 => 50_000.0,
            FuelTankType::Tank3 => 80_000.0,
            FuelTankType::Tank4 => 120_000.0,
        }
    }

    pub fn capacity_kg(&self) -> f64 {
        self.capacity_litres() * FUEL_DENSITY_KG_PER_LITRE
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuelTank {
    type_: FuelTankType,
    remaining_litres: f64,
}

impl FuelTank {
    pub fn new(type_: FuelTankType) -> Self {
        let remaining_litres = type_.capacity_litres();
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
        self.remaining_litres * FUEL_DENSITY_KG_PER_LITRE
    }

    pub fn set_fuel_kg(&mut self, new_fuel_kg: f64) {
        self.remaining_litres = new_fuel_kg / FUEL_DENSITY_KG_PER_LITRE;
    }
}