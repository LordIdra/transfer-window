use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::components::path_component::orbit::scary_math::GRAVITATIONAL_CONSTANT;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuidancePoint {
    parent_mass: f64,
    mass: f64,
    time: f64,
    rotation: f64,
    position: DVec2,
    velocity: DVec2,
    guidance_acceleration: DVec2,
    dv: f64,
}

impl GuidancePoint {
    pub fn new(parent_mass: f64, mass: f64, time: f64, rotation: f64, position: DVec2, velocity: DVec2, guidance_acceleration: DVec2, dv: f64) -> Self {
        Self { parent_mass, mass, time, rotation, position, velocity, guidance_acceleration, dv }
    }

    pub fn next(&self, delta_time: f64, new_mass: f64) -> Self {
        let gravity_acceleration = -self.position.normalize() * (GRAVITATIONAL_CONSTANT * self.parent_mass) / self.position.magnitude_squared();
        let acceleration = gravity_acceleration + self.guidance_acceleration;
        let parent_mass = self.parent_mass;
        let time = self.time + delta_time;
        let rotation = f64::atan2(acceleration.y, acceleration.x);
        let velocity = self.velocity + acceleration * delta_time;
        let position = self.position + velocity * delta_time;
        let artificial_acceleration = self.guidance_acceleration;
        let dv = self.dv + self.guidance_acceleration.magnitude() * delta_time;
        Self { parent_mass, mass: new_mass, time, rotation, position, velocity, guidance_acceleration: artificial_acceleration, dv }
    }

    pub fn next_with_new_acceleration_and_dv(&self, delta_time: f64, new_mass: f64, guidance_acceleration: DVec2) -> Self {
        let mut point = self.next(delta_time, new_mass);
        point.guidance_acceleration = guidance_acceleration;
        point
    }

    pub fn mass(&self) -> f64 {
        self.mass
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

    pub fn dv(&self) -> f64 {
        self.dv
    }
}