use serde::{Deserialize, Serialize};

use crate::components::path_component::orbit::scary_math::STANDARD_GRAVITY;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Engine {
    fuel_kg_per_second: f64,
    thrust_newtons: f64,
    specific_impulse: f64,
}

impl Engine {
    pub fn new(fuel_kg_per_second: f64, thrust_newtons: f64) -> Self {
        let specific_impulse = thrust_newtons / (STANDARD_GRAVITY * fuel_kg_per_second);
        Engine { fuel_kg_per_second, thrust_newtons, specific_impulse }
    }

    pub fn fuel_kg_per_second(&self) -> f64 {
        self.fuel_kg_per_second
    }

    pub fn thrust_newtons(&self) -> f64 {
        self.thrust_newtons
    }

    pub fn specific_impulse(&self) -> f64 {
        self.specific_impulse
    }
}
