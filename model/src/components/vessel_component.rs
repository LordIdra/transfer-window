use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use self::{system_slot::{fuel_tank::FUEL_DENSITY, Slot, SlotLocation, Slots, System}, timeline::{Timeline, TimelineEvent}};

use super::path_component::orbit::scary_math::STANDARD_GRAVITY;

pub mod system_slot;
pub mod timeline;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum VesselClass {
    Torpedo,
    Light,
}

impl VesselClass {
    pub fn mass(&self) -> f64 {
        match self {
            VesselClass::Torpedo => 1.0e3,
            VesselClass::Light => 1.0e4,
        }
    }

    pub fn is_torpedo(self) -> bool {
        matches!(self, Self::Torpedo)
    }
}

/// Must have `MassComponent` and `PathComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct VesselComponent {
    timeline: Timeline,
    class: VesselClass,
    slots: Slots,
    target: Option<Entity>,
}

#[allow(clippy::new_without_default)]
impl VesselComponent {
    pub fn new(class: VesselClass) -> Self {
        let timeline = Timeline::default();
        let slots = Slots::new(class);
        Self { timeline, class, slots, target: None }
    }

    pub fn set_target(&mut self, target: Option<Entity>) {
        self.target = target;
    }
    
    pub fn target(&self) -> Option<Entity> {
        self.target
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    pub fn class(&self) -> VesselClass {
        self.class
    }

    pub fn slots(&self) -> &Slots {
        &self.slots
    }

    pub(crate) fn slots_mut(&mut self) -> &mut Slots {
        &mut self.slots
    }

    pub(crate) fn set_slot(&mut self, location: SlotLocation, slot: Slot) {
        self.slots.set(location, slot);
    }

    pub fn remaining_fuel_litres(&self) -> f64 {
        let mut current_fuel = 0.0;
        for fuel_tank in self.slots.fuel_tanks() {
            current_fuel += fuel_tank.remaining_litres();
        }
        current_fuel
    }

    pub fn remaining_fuel_kg(&self) -> f64 {
        self.remaining_fuel_litres() * FUEL_DENSITY
    }

    pub fn max_fuel_litres(&self) -> f64 {
        let mut current_fuel = 0.0;
        for fuel_tank in self.slots.fuel_tanks() {
            current_fuel += fuel_tank.type_().capacity_litres();
        }
        current_fuel
    }

    pub fn max_fuel_kg(&self) -> f64 {
        self.max_fuel_litres() * FUEL_DENSITY
    }

    pub fn dry_mass(&self) -> f64 {
        self.class.mass()
    }

    pub fn mass(&self) -> f64 {
        self.dry_mass() + self.remaining_fuel_kg() * FUEL_DENSITY
    }

    pub fn max_dv(&self) -> Option<f64> {
        let initial_mass = self.dry_mass() + self.max_fuel_kg();
        let final_mass = self.dry_mass();
        let isp = self.slots().engine()?.type_().specific_impulse_space();
        Some(isp * STANDARD_GRAVITY * f64::ln(initial_mass / final_mass))
    }

    pub fn remaining_dv(&self) -> Option<f64> {
        let initial_mass = self.mass();
        let final_mass = self.dry_mass();
        let isp = self.slots().engine()?.type_().specific_impulse_space();
        Some(isp * STANDARD_GRAVITY * f64::ln(initial_mass / final_mass))
    }

    pub fn can_change_target(&self) -> bool {
        !self.class.is_torpedo()
    }

    pub fn can_edit(&self) -> bool {
        !self.class.is_torpedo()
    }

    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }

    pub fn timeline_mut(&mut self) -> &mut Timeline {
        &mut self.timeline
    }
}