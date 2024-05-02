use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use self::system_slot::{fuel_tank::FUEL_DENSITY, Slot, SlotLocation, Slots, System};

use super::trajectory_component::orbit::scary_math::STANDARD_GRAVITY;

pub mod system_slot;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum VesselClass {
    Light,
}

impl VesselClass {
    pub fn get_mass(&self) -> f64 {
        match self {
            VesselClass::Light => 1.0e3,
        }
    }
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

    pub fn get_slots(&self) -> &Slots {
        &self.slots
    }

    pub fn get_slots_mut(&mut self) -> &mut Slots {
        &mut self.slots
    }

    pub fn set_slot(&mut self, location: SlotLocation, slot: Slot) {
        self.slots.set(location, slot);
    }

    pub fn get_remaining_fuel_litres(&self) -> f64 {
        let mut current_fuel = 0.0;
        for fuel_tank in self.slots.get_fuel_tanks() {
            current_fuel += fuel_tank.get_remaining_litres();
        }
        current_fuel
    }

    pub fn get_remaining_fuel_kg(&self) -> f64 {
        self.get_remaining_fuel_litres() * FUEL_DENSITY
    }

    pub fn get_max_fuel_litres(&self) -> f64 {
        let mut current_fuel = 0.0;
        for fuel_tank in self.slots.get_fuel_tanks() {
            current_fuel += fuel_tank.get_type().get_capacity_litres();
        }
        current_fuel
    }

    pub fn get_max_fuel_kg(&self) -> f64 {
        self.get_max_fuel_litres() * FUEL_DENSITY
    }

    pub fn get_dry_mass(&self) -> f64 {
        self.class.get_mass()
    }

    pub fn get_mass(&self) -> f64 {
        self.get_dry_mass() + self.get_remaining_fuel_kg() * FUEL_DENSITY
    }

    pub fn get_max_dv(&self) -> Option<f64> {
        let initial_mass = self.get_dry_mass() + self.get_max_fuel_kg();
        let final_mass = self.get_dry_mass();
        let isp = self.get_slots().get_engine()?.get_type().get_specific_impulse_space();
        Some(isp * STANDARD_GRAVITY * f64::ln(initial_mass / final_mass))
    }

    pub fn get_remaining_dv(&self) -> Option<f64> {
        let initial_mass = self.get_mass();
        let final_mass = self.get_dry_mass();
        let isp = self.get_slots().get_engine()?.get_type().get_specific_impulse_space();
        Some(isp * STANDARD_GRAVITY * f64::ln(initial_mass / final_mass))
    }

    pub fn deplete_fuel(&mut self, fuel_to_deplete_kg: f64) {
        self.slots.set_fuel_kg(fuel_to_deplete_kg);
    }
}