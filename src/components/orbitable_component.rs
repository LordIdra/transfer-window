/// Must have MassComponent and either StationaryComponent or TrajectoryComponent
#[derive(Debug)]
pub struct OrbitableComponent {}

impl OrbitableComponent {
    pub fn new() -> Self {
        Self {}
    }
}