use std::collections::BTreeMap;

use log::error;
use serde::{Deserialize, Serialize};
use ship_slot::{engine::Engine, fuel_tank::{FuelTank, FUEL_DENSITY}, weapon::{Weapon, WeaponType}, ShipSlot, ShipSlotLocation, System};

use crate::storage::entity_allocator::Entity;

use super::{timeline::Timeline, Faction};

pub mod ship_slot;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ShipClass {
    Scout,
    Frigate,
}

impl ShipClass {
    pub fn name(&self) -> &'static str {
        match self {
            ShipClass::Scout => "Scout",
            ShipClass::Frigate => "Frigate",
        }
    }
    pub fn mass(&self) -> f64 {
        match self {
            Self::Scout => 10.0e3,
            Self::Frigate => 25.0e3,
        }
    }

    pub fn default_slots(&self) -> BTreeMap<ShipSlotLocation, ShipSlot> {
        match self {
            Self::Scout => [
                (ShipSlotLocation::Middle, ShipSlot::FuelTank(None)),
                (ShipSlotLocation::Back, ShipSlot::Engine(None)),
            ].into_iter().collect(),

            Self::Frigate => [
                (ShipSlotLocation::Front, ShipSlot::Weapon(None)),
                (ShipSlotLocation::Middle, ShipSlot::FuelTank(None)),
                (ShipSlotLocation::Back, ShipSlot::Engine(None)),
            ].into_iter().collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ship {
    class: ShipClass,
    faction: Faction,
    target: Option<Entity>,
    slots: BTreeMap<ShipSlotLocation, ShipSlot>,
    timeline: Timeline,
}

#[allow(clippy::new_without_default)]
impl Ship {
    pub fn new(class: ShipClass, faction: Faction) -> Self {
        let target = None;
        let timeline = Timeline::default();
        let slots = class.default_slots();
        Self { class, faction, target, slots, timeline }
    }

    pub fn faction(&self) -> Faction {
        self.faction
    }

    pub fn target(&self) -> Option<Entity> {
        self.target
    }

    pub fn set_target(&mut self, target: Option<Entity>) {
        self.target = target;
    }

    pub fn class(&self) -> ShipClass {
        self.class
    }

    pub(crate) fn set_slot(&mut self, location: ShipSlotLocation, slot: ShipSlot) {
        *self.slots.get_mut(&location).expect("Location does not contain slot") = slot;
    }

    /// # Panics
    /// Panics if the location does not contain a slot
    pub fn get_slot(&self, location: ShipSlotLocation) -> &ShipSlot {
        self.slots.get(&location).expect("Location does not contain slot")
    }

    /// # Panics
    /// Panics if the location does not contain a slot
    pub fn get_slot_mut(&mut self, location: ShipSlotLocation) -> &mut ShipSlot {
        self.slots.get_mut(&location).expect("Location does not contain slot")
    }

    pub fn filled_slot_locations(&self) -> Vec<ShipSlotLocation> {
        self.slots.keys().copied().collect()
    }

    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }

    pub fn timeline_mut(&mut self) -> &mut Timeline {
        &mut self.timeline
    }

    pub fn dry_mass(&self) -> f64 {
        self.class.mass()
    }

    pub fn wet_mass(&self) -> f64 {
        self.dry_mass() + self.max_fuel_kg()
    }

    pub fn mass(&self) -> f64 {
        self.dry_mass() + self.fuel_kg()
    }

    pub fn max_fuel_litres(&self) -> f64 {
        let mut fuel = 0.0;
        for fuel_tank in self.fuel_tanks() {
            fuel += fuel_tank.type_().capacity_litres();
        }
        fuel
    }

    pub fn max_fuel_kg(&self) -> f64 {
        self.max_fuel_litres() * FUEL_DENSITY
    }

    pub fn fuel_litres(&self) -> f64 {
        let mut fuel = 0.0;
        for fuel_tank in self.fuel_tanks() {
            fuel += fuel_tank.remaining_litres();
        }
        fuel
    }

    pub fn fuel_kg(&self) -> f64 {
        self.fuel_litres() * FUEL_DENSITY
    }

    pub fn specific_impulse(&self) -> Option<f64> {
        Some(self.engine()?.type_().specific_impulse_space())
    }

    pub fn fuel_kg_per_second(&self) -> Option<f64> {
        Some(self.engine()?.type_().fuel_kg_per_second())
    }

    /// # Panics
    /// Panics if the slot does not exist or is not a torpedo
    pub fn final_torpedoes(&self, slot_location: ShipSlotLocation) -> usize { 
        let initial_torpedoes = self.slots
            .get(&slot_location)
            .unwrap()
            .as_weapon()
            .unwrap()
            .type_()
            .as_torpedo()
            .stockpile();
        let depleted_torpedoes = self.timeline()
            .depleted_torpedoes(slot_location);
        initial_torpedoes - depleted_torpedoes
    }

    pub fn fuel_tanks(&self) -> Vec<&FuelTank> {
        let mut fuel_tanks = vec![];
        for location in self.filled_slot_locations() {
            if let ShipSlot::FuelTank(Some(fuel_tank)) = self.get_slot(location) {
                fuel_tanks.push(fuel_tank);
            }
        }
        fuel_tanks
    }

    pub fn set_fuel_kg(&mut self, new_fuel_kg: f64) {
        let fuel_tank_count = self.fuel_tanks().len();
        let kg_per_fuel_tank = new_fuel_kg / fuel_tank_count as f64;

        for location in self.filled_slot_locations() {
            if let ShipSlot::FuelTank(Some(fuel_tank)) = self.get_slot_mut(location) {
                fuel_tank.set_remaining(kg_per_fuel_tank);
            }
        }
    }

    /// For now, we assume the ship only has one engine
    /// # Panics
    /// Panics if the ship does not have an engine slot
    pub fn engine(&self) -> Option<&Engine> {
        for location in self.filled_slot_locations() {
            if let ShipSlot::Engine(engine) = self.get_slot(location) {
                return engine.as_ref();
            }
        }
        error!("A ship does not appear to have an engine slot");
        panic!("Error recoverable but exiting before something bad happens")
    }

    pub fn weapon_slots(&self) -> Vec<(ShipSlotLocation, Option<&Weapon>)> {
        let mut weapons = vec![];
        for location in self.filled_slot_locations() {
            if let ShipSlot::Weapon(weapon) = self.get_slot(location) {
                weapons.push((location, weapon.as_ref()));
            }
        }
        weapons
    }

    pub fn max_torpedoes(&self) -> usize {
        let mut max_torpedoes = 0;
        for weapon in self.weapon_slots() {
            if let Some(weapon) = weapon.1 {
                match weapon.type_() {
                    WeaponType::Torpedo(torpedo) => max_torpedoes += torpedo.max_stockpile(),
                    WeaponType::EnhancedTorpedo(enhanced_torpedo) => max_torpedoes += enhanced_torpedo.max_stockpile(),
                }
            }
        }
        max_torpedoes
    }

    pub fn torpedoes(&self) -> usize {
        let mut torpedoes = 0;
        for weapon in self.weapon_slots() {
            if let Some(weapon) = weapon.1 {
                match weapon.type_() {
                    WeaponType::Torpedo(torpedo) => torpedoes += torpedo.stockpile(),
                    WeaponType::EnhancedTorpedo(enhanced_torpedo) => torpedoes += enhanced_torpedo.stockpile(),
                }
            }
        }
        torpedoes
    }
}