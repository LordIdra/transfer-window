use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use self::{system_slot::{fuel_tank::FUEL_DENSITY, Slot, SlotLocation, Slots, System}, timeline::Timeline};

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

    pub fn name(&self) -> &str {
        match self {
            VesselClass::Torpedo => "Torpedo",
            VesselClass::Light => "Light",
        }
    }
}

/// Must have `MassComponent` and `PathComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct VesselComponent {
    ghost: bool,
    timeline: Timeline,
    class: VesselClass,
    slots: Slots,
    target: Option<Entity>,
}

#[allow(clippy::new_without_default)]
impl VesselComponent {
    pub fn new(class: VesselClass) -> Self {
        let ghost = false;
        let timeline = Timeline::default();
        let slots = Slots::new(class);
        Self { ghost, timeline, class, slots, target: None }
    }

    pub fn with_ghost(mut self) -> Self {
        self.ghost = true;
        self
    }

    pub fn is_ghost(&self) -> bool {
        self.ghost
    }

    pub fn set_ghost(&mut self, ghost: bool) {
        self.ghost = ghost;
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

    pub fn class_mut(&mut self) -> &mut VesselClass {
        &mut self.class
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

    pub fn dry_mass(&self) -> f64 {
        self.class.mass()
    }

    pub fn wet_mass(&self) -> f64 {
        self.dry_mass() + self.max_fuel_kg() * FUEL_DENSITY
    }

    pub fn mass(&self) -> f64 {
        self.dry_mass() + self.fuel_kg() * FUEL_DENSITY
    }

    pub fn max_fuel_litres(&self) -> f64 {
        let mut fuel = 0.0;
        for fuel_tank in self.slots.fuel_tanks() {
            fuel += fuel_tank.type_().capacity_litres();
        }
        fuel
    }

    pub fn max_fuel_kg(&self) -> f64 {
        self.max_fuel_litres() * FUEL_DENSITY
    }

    pub fn max_dv(&self) -> Option<f64> {
        let initial_mass = self.dry_mass() + self.max_fuel_kg();
        let final_mass = self.dry_mass();
        let isp = self.slots().engine()?.type_().specific_impulse_space();
        Some(isp * STANDARD_GRAVITY * f64::ln(initial_mass / final_mass))
    }

    pub fn fuel_litres(&self) -> f64 {
        let mut fuel = 0.0;
        for fuel_tank in self.slots.fuel_tanks() {
            fuel += fuel_tank.remaining_litres();
        }
        fuel
    }

    pub fn fuel_kg(&self) -> f64 {
        self.fuel_litres() * FUEL_DENSITY
    }

    pub fn dv(&self) -> Option<f64> {
        let initial_mass = self.mass();
        let final_mass = self.dry_mass();
        let isp = self.slots().engine()?.type_().specific_impulse_space();
        Some(isp * STANDARD_GRAVITY * f64::ln(initial_mass / final_mass))
    }

    pub fn can_edit_ever(&self) -> bool {
        !self.class.is_torpedo()
    }

    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }

    pub fn timeline_mut(&mut self) -> &mut Timeline {
        &mut self.timeline
    }

    /// # Panics
    /// Panics if the slot does not exist or is not a torpedo
    pub fn final_torpedoes(&self, slot_location: SlotLocation) -> usize { 
        let initial_torpedoes = self.slots()
            .get(slot_location)
            .as_weapon()
            .unwrap()
            .type_()
            .as_torpedo()
            .stockpile();
        let depleted_torpedoes = self.timeline()
            .depleted_torpedoes(slot_location);
        initial_torpedoes - depleted_torpedoes
    }

    pub(crate) fn should_recompute_trajectory(&self) -> bool {
        !(self.class.is_torpedo() && self.timeline.last_event().is_some_and(|event| event.is_intercept()))
    }
}