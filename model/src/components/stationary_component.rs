use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

/// Must have `MassComponent` and cannot have `TrajectoryComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct StationaryComponent {
    position: DVec2,
}

impl StationaryComponent {
    pub fn new(position: DVec2) -> Self {
        Self { position }
    }

    pub fn get_position(&self) -> DVec2 {
        self.position
    }
}