use serde::{Deserialize, Serialize};

/// Must have `MassComponent` and `TrajectoryComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct VesselComponent {}

#[allow(clippy::new_without_default)]
impl VesselComponent {
    pub fn new() -> Self {
        Self {}
    }
}