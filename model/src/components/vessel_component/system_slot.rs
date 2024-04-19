use serde::{Deserialize, Serialize};

use self::{engine::Engine, fuel_tank::FuelTank};

use super::VesselClass;

mod engine;
mod fuel_tank;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SystemSlots {
    Light(Option<FuelTank>, Option<Engine>),
    Heavy(Option<FuelTank>, Option<FuelTank>, Option<FuelTank>, Option<Engine>),
}

impl SystemSlots {
    const LIGHT_DEFAULT: SystemSlots = SystemSlots::Light(None, None);
    const HEAVY_DEFAULT: SystemSlots = SystemSlots::Heavy(None, None, None, None);

    pub fn get_default(class: &VesselClass) -> SystemSlots {
        match class {
            VesselClass::Light => Self::LIGHT_DEFAULT,
            VesselClass::Heavy => Self::HEAVY_DEFAULT,
        }
    }
}