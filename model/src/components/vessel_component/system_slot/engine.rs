use serde::{Deserialize, Serialize};

use crate::components::path_component::orbit::scary_math::STANDARD_GRAVITY;

use super::{System, SystemType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EngineType {
    Torpedo,
    Efficient,
    HighThrust,
}

impl SystemType for EngineType {}

impl EngineType {
    pub fn types() -> [Self; 2] {
        [
            EngineType::Efficient, 
            EngineType::HighThrust,
        ]
    }

    pub fn fuel_kg_per_second(&self) -> f64 {
        match self {
            EngineType::Torpedo => 10.0,
            EngineType::Efficient => 5.0,
            EngineType::HighThrust => 30.0,
        }
    }

    pub fn thrust_newtons(&self) -> f64 {
        match self {
            EngineType::Torpedo => 25000.0,
            EngineType::Efficient => 15000.0,
            EngineType::HighThrust => 75000.0,
        }
    }

    pub fn specific_impulse_space(&self) -> f64 {
        self.thrust_newtons() / (STANDARD_GRAVITY * self.fuel_kg_per_second())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Engine {
    type_: EngineType,
}

impl System for Engine {
    type Type = EngineType;
    
    fn type_(&self) -> &Self::Type {
        &self.type_
    }

    fn type_mut(&mut self) -> &mut Self::Type {
        &mut self.type_
    }
}

impl Engine {
    pub fn new(type_: EngineType) -> Self {
        Engine { type_ }
    }
}