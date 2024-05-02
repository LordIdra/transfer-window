use std::collections::BTreeMap;

use log::error;
use serde::{Deserialize, Serialize};

use self::{engine::Engine, fuel_tank::FuelTank, weapon::Weapon};

use super::VesselClass;

pub mod engine;
pub mod fuel_tank;
pub mod weapon;

pub trait System {
    type Type: SystemType;
    fn get_type(&self) -> &Self::Type;
}

pub trait SystemType {}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SlotLocation {
    Front,
    Middle,
    Back,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub(crate) fn set(&mut self, location: SlotLocation, slot: Slot) {
        *self.slots.get_mut(&location).expect("Location does not contain slot") = slot;
    }

    /// # Panics
    /// Panics if the location does not contain a slot
    pub fn get(&self, location: SlotLocation) -> &Slot {
        self.slots.get(&location).expect("Location does not contain slot")
    }

    pub fn get_filled_slot_locations(&self) -> Vec<SlotLocation> {
        self.slots.keys().cloned().into_iter().collect()
    }

    pub fn get_fuel_tanks(&self) -> Vec<&FuelTank> {
        let mut fuel_tanks = vec![];
        for location in self.get_filled_slot_locations() {
            if let Slot::FuelTank(fuel_tank) = self.get(location) {
                if let Some(fuel_tank) = fuel_tank {
                    fuel_tanks.push(fuel_tank);
                }
            }
        }
        fuel_tanks
    }

    /// For now, we assume the ship only has one engine
    pub fn get_engine(&self) -> Option<&Engine> {
        for location in self.get_filled_slot_locations() {
            if let Slot::Engine(engine) = self.get(location) {
                return engine.as_ref();
            }
        }
        error!("A ship does not appear to have an engine slot");
        panic!("Error recoverable but exiting before something bad happens")
    }
}
