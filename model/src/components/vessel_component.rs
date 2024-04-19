use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use self::system_slot::SystemSlots;

pub mod system_slot;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VesselClass {
    Light,
    Heavy,
}

/// Must have `MassComponent` and `TrajectoryComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct VesselComponent {
    class: VesselClass,
    systems: SystemSlots,
    target: Option<Entity>,
}

#[allow(clippy::new_without_default)]
impl VesselComponent {
    pub fn new(class: VesselClass) -> Self {
        let systems = SystemSlots::get_default(&class);
        Self { class, systems, target: None }
    }

    pub fn set_target(&mut self, target: Option<Entity>) {
        self.target = target;
    }
    
    pub fn get_target(&self) -> Option<Entity> {
        self.target
    }
}