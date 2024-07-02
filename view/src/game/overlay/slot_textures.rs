use transfer_window_model::components::vessel_component::ship::ship_slot::{engine::EngineType, fuel_tank::FuelTankType, weapon::WeaponType, ShipSlot, System};

pub trait TexturedSlot {
    fn texture(&self) -> &str;
}

impl TexturedSlot for EngineType {
    fn texture(&self) -> &str {
        match self {
            EngineType::Regular => "engine-regular",
            EngineType::Efficient => "engine-efficient",
            EngineType::Booster => "engine-booster",
        }
    }
}

impl TexturedSlot for FuelTankType {
    fn texture(&self) -> &str {
        match self {
            FuelTankType::Tiny => "tank-tiny",
            FuelTankType::Small => "tank-small",
            FuelTankType::Medium => "tank-medium",
        }
    }
}

impl TexturedSlot for WeaponType {
    fn texture(&self) -> &str {
        match self {
            WeaponType::Torpedo(_) => "torpedo",
            WeaponType::EnhancedTorpedo(_) => "enhanced-torpedo",
        }
    }
}

impl TexturedSlot for ShipSlot {
    fn texture(&self) -> &str {
        match self {
            ShipSlot::Weapon(weapon) => match weapon {
                None => "blank-slot",
                Some(weapon) => weapon.type_().texture(),
            },
            ShipSlot::FuelTank(fuel_tank) => match fuel_tank {
                None => "blank-slot",
                Some(fuel_tank) => fuel_tank.type_().texture(),
            },
            ShipSlot::Engine(engine) => match engine {
                None => "blank-slot",
                Some(engine) => engine.type_().texture(),
            },
        }
    }
}