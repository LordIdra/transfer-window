use serde::{Deserialize, Serialize};

/// Must have `MassComponent` and either `StationaryComponent` or `TrajectoryComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct OrbitableComponent {
    radius: f64,
}

impl OrbitableComponent {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}