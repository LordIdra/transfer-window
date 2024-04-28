use serde::{Deserialize, Serialize};

use super::{System, SystemType};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum EngineType {
    Efficient,
    HighThrust,
}

impl SystemType for EngineType {}

impl EngineType {
    pub const TYPES: [EngineType; 2] = [
        EngineType::Efficient, 
        EngineType::HighThrust,
    ];

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

impl System for Engine {
    type Type = EngineType;
    
    fn get_type(&self) -> &Self::Type {
        &self.type_
    }
}

impl Engine {
    pub fn new(type_: EngineType) -> Self {
        Engine { type_ }
    }
}