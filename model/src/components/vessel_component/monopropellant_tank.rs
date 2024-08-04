use serde::{Deserialize, Serialize};

pub const FUEL_DENSITY_KG_PER_LITRE: f64 = 1.0;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonopropellantTank {
    capacity_litres: f64,
    remaining_litres: f64,
}

impl MonopropellantTank {
    pub fn new(capacity_litres: f64) -> Self {
        let remaining_litres = capacity_litres;
        MonopropellantTank { capacity_litres, remaining_litres }
    }

    pub fn capacity_litres(&self) -> f64 {
        self.capacity_litres
    }

    pub fn capacity_kg(&self) -> f64 {
        self.capacity_litres * FUEL_DENSITY_KG_PER_LITRE
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
