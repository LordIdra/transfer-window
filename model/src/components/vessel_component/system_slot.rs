use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use self::{engine::Engine, fuel_tank::FuelTank, weapon::Weapon};

use super::VesselClass;

pub mod engine;
pub mod fuel_tank;
pub mod weapon;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SlotLocation {
    Front,
    Middle,
    Back,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Slot {
    Weapon(Option<Weapon>),
    FuelTank(Option<FuelTank>),
    Engine(Option<Engine>),
}

/// Maintains a mapping of locations to slots. The purpose of this 
/// is to provide a wrapper that does not allow new slots to be 
/// inserted at locations where the class should not have slots. In
/// other words, this ensures that the schema for slots for a
/// particular class is respected.
#[derive(Debug, Serialize, Deserialize)]
pub struct Slots {
    slots: BTreeMap<SlotLocation, Slot>
}

impl Slots {
    pub fn new(class: VesselClass) -> Self {
        let slots = match class {
            VesselClass::Light => [
                (SlotLocation::Front, Slot::Weapon(None)),
                (SlotLocation::Middle, Slot::FuelTank(None)),
                (SlotLocation::Back, Slot::Engine(None)),
            ].into_iter().collect(),
        };
        Self { slots }
    }

    /// # Panics
    /// Panics if the location does not contain a slot
    pub fn get(&self, location: SlotLocation) -> &Slot {
        self.slots.get(&location).expect("Location does not contain slot")
    }
}