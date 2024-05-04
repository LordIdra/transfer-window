use std::collections::BTreeMap;

use transfer_window_model::components::vessel_component::{system_slot::{engine::EngineType, fuel_tank::FuelTankType, weapon::WeaponType, Slot, SlotLocation, System}, VesselClass};

pub fn compute_slot_size(vessel_class: VesselClass) -> f32 {
    match vessel_class {
        VesselClass::Light => 0.113,
    }
}

pub fn compute_slot_locations(vessel_class: VesselClass) -> BTreeMap<SlotLocation, f32> {
    match vessel_class {
        VesselClass::Light => vec![
            (SlotLocation::Front, -0.142),
            (SlotLocation::Middle, 0.177),
            (SlotLocation::Back, 0.382),
        ].into_iter().collect(),
    }
}

pub trait TexturedSlot {
    fn texture(&self) -> &str;
}

impl TexturedSlot for EngineType {
    fn texture(&self) -> &str {
        match self {
            EngineType::Efficient => "engine-efficient",
            EngineType::HighThrust => "engine-high-thrust",
        }
    }
}

impl TexturedSlot for FuelTankType {
    fn texture(&self) -> &str {
        match self {
            FuelTankType::Small => "tank-small",
            FuelTankType::Medium => "tank-medium",
            FuelTankType::Large => "tank-large",
        }
    }
}

impl TexturedSlot for WeaponType {
    fn texture(&self) -> &str {
        match self {
            WeaponType::Torpedo => "torpedo",
        }
    }
}

impl TexturedSlot for Slot {
    fn texture(&self) -> &str {
        match self {
            Slot::Weapon(weapon) => match weapon {
                None => "blank-slot",
                Some(weapon) => weapon.type_().texture(),
            },
            Slot::FuelTank(fuel_tank) => match fuel_tank {
                None => "blank-slot",
                Some(fuel_tank) => fuel_tank.type_().texture(),
            },
            Slot::Engine(engine) => match engine {
                None => "blank-slot",
                Some(engine) => engine.type_().texture(),
            },
        }
    }
}