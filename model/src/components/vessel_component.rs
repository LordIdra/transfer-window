use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use self::system_slot::Slots;

pub mod system_slot;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum VesselClass {
    Light,
}

/// Must have `MassComponent` and `TrajectoryComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct VesselComponent {
    class: VesselClass,
    slots: Slots,
    target: Option<Entity>,
}

#[allow(clippy::new_without_default)]
impl VesselComponent {
    pub fn new(class: VesselClass) -> Self {
        let slots = Slots::new(class);
        Self { class, slots, target: None }
    }

    pub fn set_target(&mut self, target: Option<Entity>) {
        self.target = target;
    }
    
    pub fn get_target(&self) -> Option<Entity> {
        self.target
    }

    pub fn class(&self) -> VesselClass {
        self.class
    }

    pub fn slots(&self) -> &Slots {
        &self.slots
    }
}