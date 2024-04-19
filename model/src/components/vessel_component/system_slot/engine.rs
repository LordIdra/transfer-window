use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Engine {
    name: String,
    fuel_per_second: f64,
    thrust_newtons: f64,
}

impl Engine {
    pub fn efficient_engine() -> Engine{
        Engine { name: "Efficient engine".to_string(), fuel_per_second: 50.0, thrust_newtons: 15000.0 }
    }
    
    pub fn high_thrust_engine() -> Engine{
        Engine { name: "High thrust engine".to_string(), fuel_per_second: 200.0, thrust_newtons: 75000.0 }
    }
}