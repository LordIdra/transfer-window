use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurnPoint {
    time: f64,
    position: DVec2,
    velocity: DVec2,
    rotation: f64,
    mass: f64,
}

impl TurnPoint {
    pub fn new(time: f64, position: DVec2, velocity: DVec2, rotation: f64, mass: f64) -> Self {
        Self { time, position, velocity, rotation, mass }
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

    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }
}

