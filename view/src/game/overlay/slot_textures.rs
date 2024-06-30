use transfer_window_model::components::vessel_component::{engine::EngineType, fuel_tank::FuelTankType, torpedo_launcher::TorpedoLauncherType, torpedo_storage::TorpedoStorageType};

pub trait TexturedSlot {
    fn texture(&self) -> &'static str;
}

impl TexturedSlot for EngineType {
    fn texture(&self) -> &'static str {
        match self {
            EngineType::Regular => "engine-regular",
            EngineType::Efficient => "engine-efficient",
            EngineType::Booster => "engine-booster",
            EngineType::Torpedo => panic!("Attempt to get torpedo engine texture"),
        }
    }
}

impl TexturedSlot for FuelTankType {
    fn texture(&self) -> &'static str {
        match self {
            FuelTankType::Tiny => "tank-tiny",
            FuelTankType::Small => "tank-small",
            FuelTankType::Medium => "tank-medium",
            FuelTankType::Torpedo => panic!("Attempt to get torpedo fuel tank texture"),
            FuelTankType::Hub => panic!("Attempt to get hub fuel tank texture"),
        }
    }
}

impl TexturedSlot for TorpedoStorageType {
    fn texture(&self) -> &'static str {
        match self {
            TorpedoStorageType::Tiny => "torpedo-storage-tiny",
            TorpedoStorageType::Small => "torpedo-storage-small",
            TorpedoStorageType::Hub => panic!("Attempt to get hub torpedo storage texture"),
        }
    }
}

impl TexturedSlot for TorpedoLauncherType {
    fn texture(&self) -> &'static str {
        match self {
            TorpedoLauncherType::Simple => "torpedo-launcher-simple",
            TorpedoLauncherType::Enhanced => "torpedo-launcher-enhanced",
        }
    }
}
