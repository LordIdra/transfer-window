use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuelTank {
    name: String,
    capacity: f64,
    remaining: f64,
}

impl FuelTank {
    pub fn small_tank() -> FuelTank {
        FuelTank { name: "Small Tank".to_string(), capacity: 10000.0, remaining: 10000.0 }
    }
    
    pub fn medium_tank() -> FuelTank {
        FuelTank { name: "Medium Tank".to_string(), capacity: 15000.0, remaining: 15000.0 }
    }

    pub fn large_tank() -> FuelTank {
        FuelTank { name: "Large Tank".to_string(), capacity: 20000.0, remaining: 20000.0 }
    }
}