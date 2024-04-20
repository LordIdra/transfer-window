use serde::{Deserialize, Serialize};

use self::{engine::Engine, fuel_tank::FuelTank, weapon::Weapon};

use super::VesselClass;

pub mod engine;
pub mod fuel_tank;
pub mod weapon;

#[derive(Debug, Serialize, Deserialize)]
pub enum SystemSlots {
    Light(LightSlots),
}

impl SystemSlots {
    pub fn new(class: VesselClass) -> SystemSlots {
        match class {
            VesselClass::Light => SystemSlots::Light(LightSlots::default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LightSlots {
    weapon: Option<Weapon>, 
    fuel_tank: Option<FuelTank>, 
    engine: Option<Engine>,
}

impl LightSlots {
    pub fn weapon(&self) -> Option<&Weapon> {
        self.weapon.as_ref()
    }
    
    pub fn fuel_tank(&self) -> Option<&FuelTank> {
        self.fuel_tank.as_ref()
    }
    
    pub fn engine(&self) -> Option<&Engine> {
        self.engine.as_ref()
    }
}