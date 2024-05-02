use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::components::trajectory_component::orbit::scary_math::GRAVITATIONAL_CONSTANT;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BurnPoint {
    parent_mass: f64,
    mass: f64,
    time: f64,
    position: DVec2,
    velocity: DVec2,
}

impl BurnPoint {
    pub fn new(parent_mass: f64, mass: f64, time: f64, position: DVec2, velocity: DVec2) -> Self {
        Self { parent_mass, mass, time, position, velocity }
    }

    pub fn next(&self, delta_time: f64, new_mass: f64, artificial_acceleration: DVec2) -> Self {
        let gravity_acceleration = -self.position.normalize() * (GRAVITATIONAL_CONSTANT * self.parent_mass) / self.position.magnitude_squared();
        let acceleration = gravity_acceleration + artificial_acceleration;
        let parent_mass = self.parent_mass;
        let time = self.time + delta_time;
        let velocity = self.velocity + acceleration * delta_time;
        let position = self.position + velocity * delta_time;
        Self { parent_mass, mass: new_mass, time, position, velocity }
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_position(&self) -> DVec2 {
        self.position
    }

    pub fn get_velocity(&self) -> DVec2 {
        self.velocity
    }
}