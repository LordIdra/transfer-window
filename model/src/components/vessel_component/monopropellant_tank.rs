use serde::{Deserialize, Serialize};

pub const FUEL_DENSITY_KG_PER_LITRE: f64 = 1.0;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MonopropellantTankType {
    Torpedo,
    Hub,
    Tank1,
    Tank2,
    Tank3,
    Tank4,
}

impl MonopropellantTankType {
    pub fn mass(&self) -> f64 {
        match self {
            MonopropellantTankType::Torpedo => 5.0,
            MonopropellantTankType::Hub => 20.0,
            MonopropellantTankType::Tank1 => 7.0,
            MonopropellantTankType::Tank2 => 10.0,
            MonopropellantTankType::Tank3 => 12.0,
            MonopropellantTankType::Tank4 => 13.0,
        }
    }

    pub fn ship_types() -> [Self; 4] {
        [
            MonopropellantTankType::Tank1,
            MonopropellantTankType::Tank2,
            MonopropellantTankType::Tank3,
            MonopropellantTankType::Tank4,
        ]
    }

    pub fn capacity_litres(&self) -> f64 {
        match self {
            MonopropellantTankType::Torpedo => 100.0,
            MonopropellantTankType::Hub => 1400.0,
            MonopropellantTankType::Tank1 => 300.0,
            MonopropellantTankType::Tank2 => 500.0,
            MonopropellantTankType::Tank3 => 800.0,
            MonopropellantTankType::Tank4 => 1200.0,
        }
    }

    pub fn capacity_kg(&self) -> f64 {
        self.capacity_litres() * FUEL_DENSITY_KG_PER_LITRE
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonopropellantTank {
    type_: MonopropellantTankType,
    remaining_litres: f64,
}

impl MonopropellantTank {
    pub fn new(type_: MonopropellantTankType) -> Self {
        let remaining_litres = type_.capacity_litres();
        MonopropellantTank { type_, remaining_litres }
    }

    pub fn type_(&self) -> MonopropellantTankType {
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
