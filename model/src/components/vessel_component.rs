use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

/// Must have `MassComponent` and `TrajectoryComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct VesselComponent {
    target: Option<Entity>
}

#[allow(clippy::new_without_default)]
impl VesselComponent {
    pub fn new() -> Self {
        Self { target: None }
    }

    pub fn set_target(&mut self, target: Option<Entity>) {
        self.target = target;
    }
    
    pub fn get_target(&self) -> Option<Entity> {
        self.target
    }
}