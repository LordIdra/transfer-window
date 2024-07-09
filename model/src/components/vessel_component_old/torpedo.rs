use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use super::{faction::Faction, ship::ship_slot::fuel_tank::FUEL_DENSITY, timeline::Timeline};

#[derive(Debug, Serialize, Deserialize)]
pub struct Torpedo {
    ghost: bool,
    faction: Faction,
    target: Option<Entity>,
    timeline: Timeline,
    fuel_litres: f64,
}

impl Torpedo {
    pub fn new(faction: Faction) -> Self {
        let ghost = true;
        let target = None;
        let timeline = Timeline::default();
        let fuel_litres = Self::max_fuel_litres();
        Self { ghost, faction, target, timeline, fuel_litres }
    }

    pub fn is_ghost(&self) -> bool {
        self.ghost
    }

    pub fn set_ghost(&mut self, ghost: bool) {
        self.ghost = ghost;
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

    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }

    pub fn timeline_mut(&mut self) -> &mut Timeline {
        &mut self.timeline
    }

    pub fn dry_mass() -> f64 {
        2.0e3
    }

    pub fn wet_mass() -> f64 {
        Self::dry_mass() + Self::max_fuel_kg()
    }

    pub fn mass(&self) -> f64 {
        Self::dry_mass() + self.fuel_kg()
    }

    pub fn max_fuel_litres() -> f64 {
        2000.0
    }

    pub fn max_fuel_kg() -> f64 {
        Self::max_fuel_litres() * FUEL_DENSITY
    }

    pub fn fuel_litres(&self) -> f64 {
        self.fuel_litres
    }

    pub fn fuel_kg(&self) -> f64 {
        self.fuel_litres * FUEL_DENSITY
    }

    pub fn specific_impulse() -> f64 {
        257.0
    }

    pub fn fuel_kg_per_second() -> f64 {
        10.0
    }

    pub fn set_fuel_kg(&mut self, new_fuel_kg: f64) {
        self.fuel_litres = new_fuel_kg / FUEL_DENSITY;
    }
}