use faction::Faction;
use serde::{Deserialize, Serialize};
use ship::{Ship, ShipClass};
use timeline::Timeline;
use torpedo::Torpedo;

use crate::storage::entity_allocator::Entity;

use super::path_component::orbit::scary_math::STANDARD_GRAVITY;

pub mod faction;
pub mod ship;
pub mod timeline;
mod torpedo;

/// Must have `MassComponent` and `PathComponent`
#[derive(Debug, Serialize, Deserialize)]
pub enum VesselComponent {
    Ship(Ship),
    Torpedo(Torpedo),
}

#[allow(clippy::new_without_default)]
impl VesselComponent {
    pub fn new_ship(class: ShipClass, faction: Faction) -> Self {
        Self::Ship(Ship::new(class, faction))
    }

    pub fn new_torpedo(faction: Faction) -> Self {
        Self::Torpedo(Torpedo::new(faction))
    }

    pub fn set_target(&mut self, target: Option<Entity>) {
        match self {
            VesselComponent::Ship(ship) => ship.set_target(target),
            VesselComponent::Torpedo(torpedo) => torpedo.set_target(target),
        }
    }
    
    pub fn target(&self) -> Option<Entity> {
        match self {
            VesselComponent::Ship(ship) => ship.target(),
            VesselComponent::Torpedo(torpedo) => torpedo.target(),
        }
    }

    pub fn has_target(&self) -> bool {
        self.target().is_some()
    }

    pub fn faction(&self) -> Faction {
        match self {
            VesselComponent::Ship(ship) => ship.faction(),
            VesselComponent::Torpedo(torpedo) => torpedo.faction(),
        }
    }

    pub fn dry_mass(&self) -> f64 {
        match self {
            VesselComponent::Ship(ship) => ship.dry_mass(),
            VesselComponent::Torpedo(_) => Torpedo::dry_mass(),
        }
    }

    pub fn wet_mass(&self) -> f64 {
        match self {
            VesselComponent::Ship(ship) => ship.wet_mass(),
            VesselComponent::Torpedo(_) => Torpedo::wet_mass(),
        }
    }

    pub fn mass(&self) -> f64 {
        match self {
            VesselComponent::Ship(ship) => ship.mass(),
            VesselComponent::Torpedo(torpedo) => torpedo.mass(),
        }
    }

    pub fn max_fuel_litres(&self) -> f64 {
        match self {
            VesselComponent::Ship(ship) => ship.max_fuel_litres(),
            VesselComponent::Torpedo(_) => Torpedo::max_fuel_litres(),
        }
    }

    pub fn max_fuel_kg(&self) -> f64 {
        match self {
            VesselComponent::Ship(ship) => ship.max_fuel_kg(),
            VesselComponent::Torpedo(_) => Torpedo::max_fuel_kg(),
        }
    }

    pub fn max_dv(&self) -> Option<f64> {
        let initial_mass = self.dry_mass() + self.max_fuel_kg();
        let final_mass = self.dry_mass();
        let isp = self.specific_impulse()?;
        Some(isp * STANDARD_GRAVITY * f64::ln(initial_mass / final_mass))
    }

    pub fn fuel_litres(&self) -> f64 {
        match self {
            VesselComponent::Ship(ship) => ship.fuel_litres(),
            VesselComponent::Torpedo(torpedo) => torpedo.fuel_litres(),
        }
    }

    pub fn specific_impulse(&self) -> Option<f64> {
        match self {
            VesselComponent::Ship(ship) => ship.specific_impulse(),
            VesselComponent::Torpedo(_) => Some(Torpedo::specific_impulse()),
        }
    }

    pub fn fuel_kg_per_second(&self) -> Option<f64> {
        match self {
            VesselComponent::Ship(ship) => ship.fuel_kg_per_second(),
            VesselComponent::Torpedo(_) => Some(Torpedo::fuel_kg_per_second()),
        }
    }

    pub fn fuel_kg(&self) -> f64 {
        match self {
            VesselComponent::Ship(ship) => ship.fuel_kg(),
            VesselComponent::Torpedo(torpedo) => torpedo.fuel_kg(),
        }
    }

    pub fn set_fuel_kg(&mut self, new_fuel_kg: f64) {
        match self {
            VesselComponent::Ship(ship) => ship.set_fuel_kg(new_fuel_kg),
            VesselComponent::Torpedo(torpedo) => torpedo.set_fuel_kg(new_fuel_kg),
        }
    }

    pub fn dv(&self) -> Option<f64> {
        let initial_mass = self.mass();
        let final_mass = self.dry_mass();
        let isp = self.specific_impulse()?;
        Some(isp * STANDARD_GRAVITY * f64::ln(initial_mass / final_mass))
    }

    pub fn can_edit_ever(&self) -> bool {
        match self {
            VesselComponent::Ship(_) => true,
            VesselComponent::Torpedo(_) => false,
        }
    }

    pub fn timeline(&self) -> &Timeline {
        match self {
            VesselComponent::Ship(ship) => ship.timeline(),
            VesselComponent::Torpedo(torpedo) => torpedo.timeline(),
        }
    }

    pub fn timeline_mut(&mut self) -> &mut Timeline {
        match self {
            VesselComponent::Ship(ship) => ship.timeline_mut(),
            VesselComponent::Torpedo(torpedo) => torpedo.timeline_mut(),
        }
    }

    pub(crate) fn should_recompute_trajectory(&self) -> bool {
        !(self.as_torpedo().is_some() && self.timeline().last_event().is_some_and(|event| event.is_intercept()))
    }

    pub fn as_ship(&self) -> Option<&Ship> {
        match self {
            VesselComponent::Ship(ref ship) => Some(ship),
            VesselComponent::Torpedo(_) => None,
        }
    }

    pub fn as_ship_mut(&mut self) -> Option<&mut Ship> {
        match self {
            VesselComponent::Ship(ref mut ship) => Some(ship),
            VesselComponent::Torpedo(_) => None,
        }
    }

    pub fn as_torpedo(&self) -> Option<&Torpedo> {
        match self {
            VesselComponent::Ship(_) => None,
            VesselComponent::Torpedo(ref torpedo) => Some(torpedo),
        }
    }

    pub fn as_torpedo_mut(&mut self) -> Option<&mut Torpedo> {
        match self {
            VesselComponent::Ship(_) => None,
            VesselComponent::Torpedo(ref mut torpedo) => Some(torpedo),
        }
    }

    pub fn has_engine(&self) -> bool {
        self.specific_impulse().is_some()
    }

    pub fn has_fuel_tank(&self) -> bool {
        self.max_fuel_kg() != 0.0
    }

    pub fn is_ghost(&self) -> bool {
        match self {
            VesselComponent::Ship(_) => false,
            VesselComponent::Torpedo(torpedo) => torpedo.is_ghost(),
        }
    }
}