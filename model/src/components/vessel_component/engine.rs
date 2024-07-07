use serde::{Deserialize, Serialize};

use crate::components::path_component::orbit::scary_math::STANDARD_GRAVITY;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum EngineType {
    Torpedo,
    Regular,
    Efficient,
    Booster,
}

impl EngineType {
    pub fn ship_types() -> [Self; 3] {
        [
            EngineType::Regular,
            EngineType::Efficient, 
            EngineType::Booster,
        ]
    }

    #[allow(clippy::match_same_arms)]
    pub fn fuel_kg_per_second(&self) -> f64 {
        match self {
            EngineType::Torpedo => 7.0,
            EngineType::Regular => 15.0,
            EngineType::Efficient => 7.0,
            EngineType::Booster => 60.0,
        }
    }

    pub fn thrust_newtons(&self) -> f64 {
        match self {
            EngineType::Torpedo => 15_000.0,
            EngineType::Regular => 35_000.0,
            EngineType::Efficient => 18_000.0,
            EngineType::Booster => 160_000.0,
        }
    }

    pub fn specific_impulse(&self) -> f64 {
        self.thrust_newtons() / (STANDARD_GRAVITY * self.fuel_kg_per_second())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Engine {
    type_: EngineType,
}

impl Engine {
    pub fn new(type_: EngineType) -> Self {
        Engine { type_ }
    }
    
    pub fn type_(&self) -> EngineType {
        self.type_
    }

    pub fn fuel_kg_per_second(&self) -> f64 {
        self.type_.fuel_kg_per_second()
    }

    pub fn thrust_newtons(&self) -> f64 {
        self.type_.thrust_newtons()
    }

    pub fn specific_impulse(&self) -> f64 {
        self.type_.specific_impulse()
    }
}