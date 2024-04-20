use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EngineType {
    Efficient,
    HighThrust,
}

impl EngineType {
    pub fn get_fuel_per_second(&self) -> f64 {
        match self {
            EngineType::Efficient => 50.0,
            EngineType::HighThrust => 200.0,
        }
    }

    pub fn get_thrust_newtons(&self) -> f64 {
        match self {
            EngineType::Efficient => 15000.0,
            EngineType::HighThrust => 75000.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Engine {
    type_: EngineType,
}

impl Engine {
    pub fn new(type_: EngineType) -> Self {
        Engine { 
            type_: type_.clone(), 
        }
    }
    
    pub fn type_(&self) -> &EngineType {
        &self.type_
    }
}