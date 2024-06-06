use std::collections::BTreeMap;

use log::error;
use serde::{Deserialize, Serialize};

use self::{engine::{Engine, EngineType}, fuel_tank::{FuelTank, FuelTankType}, weapon::{Weapon, WeaponType}};

use super::VesselClass;

pub mod engine;
pub mod fuel_tank;
pub mod weapon;

pub trait System {
    type Type: SystemType;
    fn type_(&self) -> &Self::Type;
    fn type_mut(&mut self) -> &mut Self::Type;
}

pub trait SystemType {}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SlotLocation {
    TorpedoEngine,
    TorpedoFuelTank,
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

impl Slot {
    pub fn is_weapon(&self) -> bool {
        matches!(self, Slot::Weapon(_))
    }

    pub fn is_fuel_tank(&self) -> bool {
        matches!(self, Slot::FuelTank(_))
    }

    pub fn is_engine(&self) -> bool {
        matches!(self, Slot::Engine(_))
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn as_weapon(&self) -> Option<&Weapon> {
        match self {
            Slot::Weapon(weapon) => weapon.as_ref(),
            _ => panic!("Attempt to get non-weapon slot as weapon slot"),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn as_weapon_mut(&mut self) -> Option<&mut Weapon> {
        match self {
            Slot::Weapon(weapon) => weapon.as_mut(),
            _ => panic!("Attempt to get non-weapon slot as weapon slot"),
        }
    }
    
    #[allow(clippy::missing_panics_doc)]
    pub fn as_fuel_tank(&self) -> Option<&FuelTank> {
        match self {
            Slot::FuelTank(fuel_tank) => fuel_tank.as_ref(),
            _ => panic!("Attempt to get non-fuel-tank slot as fuel-tank slot"),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn as_engine(&self) -> Option<&Engine> {
        match self {
            Slot::Engine(engine) => engine.as_ref(),
            _ => panic!("Attempt to get non-engine slot as engine slot"),
        }
    }

    pub fn new_weapon(type_: WeaponType) -> Self {
        Self::Weapon(Some(Weapon::new(type_)))
    }

    pub fn new_fuel_tank(type_: FuelTankType) -> Self {
        Self::FuelTank(Some(FuelTank::new(type_)))
    }

    pub fn new_engine(type_: EngineType) -> Self {
        Self::Engine(Some(Engine::new(type_)))
    }
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
            VesselClass::Torpedo => [
                (SlotLocation::TorpedoFuelTank, Slot::new_fuel_tank(FuelTankType::Torpedo)),
                (SlotLocation::TorpedoEngine, Slot::new_engine(EngineType::Torpedo)),
            ].into_iter().collect(),

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

    /// # Panics
    /// Panics if the location does not contain a slot
    pub fn get_mut(&mut self, location: SlotLocation) -> &mut Slot {
        self.slots.get_mut(&location).expect("Location does not contain slot")
    }

    pub fn filled_slot_locations(&self) -> Vec<SlotLocation> {
        self.slots.keys().copied().collect()
    }

    pub fn fuel_tanks(&self) -> Vec<&FuelTank> {
        let mut fuel_tanks = vec![];
        for location in self.filled_slot_locations() {
            if let Slot::FuelTank(Some(fuel_tank)) = self.get(location) {
                fuel_tanks.push(fuel_tank);
            }
        }
        fuel_tanks
    }

    pub fn set_fuel_kg(&mut self, new_fuel_kg: f64) {
        let fuel_tank_count = self.fuel_tanks().len();
        let kg_per_fuel_tank = new_fuel_kg / fuel_tank_count as f64;

        for location in self.filled_slot_locations() {
            if let Slot::FuelTank(Some(fuel_tank)) = self.get_mut(location) {
                fuel_tank.set_remaining(kg_per_fuel_tank);
            }
        }
    }

    /// For now, we assume the ship only has one engine
    /// # Panics
    /// Panics if the ship does not have an engine slot
    pub fn engine(&self) -> Option<&Engine> {
        for location in self.filled_slot_locations() {
            if let Slot::Engine(engine) = self.get(location) {
                return engine.as_ref();
            }
        }
        error!("A ship does not appear to have an engine slot");
        panic!("Error recoverable but exiting before something bad happens")
    }

    pub fn weapon_slots(&self) -> Vec<(SlotLocation, Option<&Weapon>)> {
        let mut weapons = vec![];
        for location in self.filled_slot_locations() {
            if let Slot::Weapon(weapon) = self.get(location) {
                weapons.push((location, weapon.as_ref()));
            }
        }
        weapons
    }

    pub fn max_torpedoes(&self) -> usize {
        let mut max_torpedoes = 0;
        for weapon in self.weapon_slots() {
            if let Some(weapon) = weapon.1 {
                #[allow(irrefutable_let_patterns)]
                if let WeaponType::Torpedo(torpedo) = weapon.type_() {
                    max_torpedoes += torpedo.max_stockpile();
                }
            }
        }
        max_torpedoes
    }

    pub fn torpedoes(&self) -> usize {
        let mut torpedoes = 0;
        for weapon in self.weapon_slots() {
            if let Some(weapon) = weapon.1 {
                #[allow(irrefutable_let_patterns)]
                if let WeaponType::Torpedo(torpedo) = weapon.type_() {
                    torpedoes += torpedo.stockpile();
                }
            }
        }
        torpedoes
    }
}
