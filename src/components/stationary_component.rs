use nalgebra_glm::DVec2;

/// Must have MassComponent and cannot have TrajectoryComponent
pub struct StationaryComponent {
    position: DVec2,
}

impl StationaryComponent {
    pub fn new(position: DVec2) -> Self {
        Self { position }
    }
}