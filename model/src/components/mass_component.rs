use serde::{Deserialize, Serialize};

/// Must have either `StationaryComponent` or `TrajectoryComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct MassComponent {
    mass: f64,
}

impl MassComponent {
    pub fn new(mass: f64,) -> Self {
        Self { mass }
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }
}