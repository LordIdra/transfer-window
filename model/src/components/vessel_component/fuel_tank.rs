use serde::{Deserialize, Serialize};

pub const FUEL_DENSITY_KG_PER_LITRE: f64 = 1.0; // both RP-1 and LOX are very roughly 1.0kg/L

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuelTank {
    capacity_litres: f64,
    remaining_litres: f64,
}

impl FuelTank {
    pub fn new(capacity_litres: f64) -> Self {
        let remaining_litres = capacity_litres;
        FuelTank { capacity_litres, remaining_litres }
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