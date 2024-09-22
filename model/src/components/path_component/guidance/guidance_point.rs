use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::components::{path_component::orbit::scary_math::GRAVITATIONAL_CONSTANT, vessel_component::engine::Engine};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuidancePoint {
    parent_mass: f64,
    dry_mass: f64,
    fuel_kg: f64,
    time: f64,
    rotation: f64,
    position: DVec2,
    velocity: DVec2,
    guidance_acceleration: DVec2,
}

impl GuidancePoint {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        parent_mass: f64, 
        dry_mass: f64, 
        fuel_kg: f64,
        time: f64, 
        rotation: f64, 
        position: DVec2, 
        velocity: DVec2, 
        guidance_acceleration: DVec2, 
    ) -> Self {
        Self { parent_mass, dry_mass, fuel_kg, time, rotation, position, velocity, guidance_acceleration, }
    }

    pub fn next(&self, delta_time: f64, engine: &Engine) -> Self {
        let parent_mass = self.parent_mass;
        let dry_mass = self.dry_mass;
        let fuel_kg = self.fuel_kg - engine.fuel_kg_per_second() * delta_time;
        let time = self.time + delta_time;

        let gravity_acceleration = -self.position.normalize() * (GRAVITATIONAL_CONSTANT * self.parent_mass) / self.position.magnitude_squared();
        let acceleration = gravity_acceleration + self.guidance_acceleration;

        let rotation = f64::atan2(acceleration.y, acceleration.x);
        let velocity = self.velocity + acceleration * delta_time;
        let position = self.position + velocity * delta_time;
        let artificial_acceleration = self.guidance_acceleration;

        Self { parent_mass, dry_mass, fuel_kg, time, rotation, position, velocity, guidance_acceleration: artificial_acceleration }
    }

    pub(super) fn set_acceleration(&mut self, artificial_acceleration: DVec2) {
        self.guidance_acceleration = artificial_acceleration;
    }

    pub fn mass(&self) -> f64 {
        self.dry_mass + self.fuel_kg
    }

    pub fn dry_mass(&self) -> f64 {
        self.dry_mass
    }

    pub fn fuel_kg(&self) -> f64 {
        self.fuel_kg
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    pub fn position(&self) -> DVec2 {
        self.position
    }

    pub fn velocity(&self) -> DVec2 {
        self.velocity
    }
    
    pub fn guidance_acceleration(&self) -> DVec2 {
        self.guidance_acceleration
    }
}
