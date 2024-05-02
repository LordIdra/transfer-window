use serde::{Deserialize, Serialize};

/// Must have `MassComponent` and either `StationaryComponent` or `TrajectoryComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct OrbitableComponent {
    mass: f64,
    radius: f64,
}

impl OrbitableComponent {
    pub fn new(mass: f64, radius: f64) -> Self {
        Self { mass, radius }
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}