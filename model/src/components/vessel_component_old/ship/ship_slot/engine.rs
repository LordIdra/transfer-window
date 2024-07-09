use serde::{Deserialize, Serialize};

use crate::components::path_component::orbit::scary_math::STANDARD_GRAVITY;

use super::{System, SystemType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EngineType {
    Regular,
    Efficient,
    Booster,
}

impl SystemType for EngineType {}

impl EngineType {
    pub fn types() -> [Self; 3] {
        [
            EngineType::Regular,
            EngineType::Efficient, 
            EngineType::Booster,
        ]
    }

    pub fn fuel_kg_per_second(&self) -> f64 {
        match self {
            EngineType::Regular => 10.0,
            EngineType::Efficient => 5.0,
            EngineType::Booster => 30.0,
        }
    }

    pub fn thrust_newtons(&self) -> f64 {
        match self {
            EngineType::Regular => 25_000.0,
            EngineType::Efficient => 18_000.0,
            EngineType::Booster => 80_000.0,
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