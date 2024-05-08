use transfer_window_model::components::vessel_component::system_slot::{engine::EngineType, fuel_tank::FuelTankType, weapon::WeaponType, Slot, System};

pub trait TexturedSlot {
    fn texture(&self) -> &str;
}

impl TexturedSlot for EngineType {
    fn texture(&self) -> &str {
        match self {
            EngineType::Torpedo => unreachable!(),
            EngineType::Efficient => "engine-efficient",
            EngineType::HighThrust => "engine-high-thrust",
        }
    }
}

impl TexturedSlot for FuelTankType {
    fn texture(&self) -> &str {
        match self {
            FuelTankType::Torpedo => unreachable!(),
            FuelTankType::Small => "tank-small",
            FuelTankType::Medium => "tank-medium",
            FuelTankType::Large => "tank-large",
        }
    }
}

impl TexturedSlot for WeaponType {
    fn texture(&self) -> &str {
        match self {
            WeaponType::Torpedo { stockpile: _, cooldown_seconds: _ } => "torpedo",
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