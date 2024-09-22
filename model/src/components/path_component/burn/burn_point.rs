use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::components::{path_component::orbit::scary_math::GRAVITATIONAL_CONSTANT, vessel_component::engine::Engine};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BurnPoint {
    parent_mass: f64,
    mass_without_fuel: f64,
    fuel_kg: f64,
    time: f64,
    position: DVec2,
    velocity: DVec2,
}

impl BurnPoint {
    pub fn new(parent_mass: f64, mass_without_fuel: f64, fuel_kg: f64, time: f64, position: DVec2, velocity: DVec2) -> Self {
        Self { parent_mass, mass_without_fuel, fuel_kg, time, position, velocity }
    }

    pub fn next(&self, delta_time: f64, engine: &Engine, absolute_delta_v: DVec2) -> Self {
        let gravity_acceleration = -self.position.normalize() * (GRAVITATIONAL_CONSTANT * self.parent_mass) / self.position.magnitude_squared();
        let artificial_acceleration = absolute_delta_v.normalize() * engine.thrust_newtons() / self.mass();
        let acceleration = gravity_acceleration + artificial_acceleration;
        let parent_mass = self.parent_mass;
        let mass_without_fuel = self.mass_without_fuel;
        let fuel_kg = self.fuel_kg - delta_time * engine.fuel_kg_per_second();
        let time = self.time + delta_time;
        let velocity = self.velocity + acceleration * delta_time;
        let position = self.position + velocity * delta_time;
        Self { parent_mass, mass_without_fuel, fuel_kg, time, position, velocity }
    }

    pub fn mass(&self) -> f64 {
        self.mass_without_fuel + self.fuel_kg
    }

    pub fn mass_without_fuel(&self) -> f64 {
        self.mass_without_fuel
    }

    pub fn fuel_kg(&self) -> f64 {
        self.fuel_kg
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn position(&self) -> DVec2 {
        self.position
    }

    pub fn velocity(&self) -> DVec2 {
        self.velocity
    }
}
