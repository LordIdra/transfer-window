use serde::{Deserialize, Serialize};

/// Must have `MassComponent` and `TrajectoryComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct VesselComponent {}

impl VesselComponent {
    pub fn new() -> Self {
        Self {}
    }
}