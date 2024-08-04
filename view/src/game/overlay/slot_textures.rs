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
            FuelTankType::Tank1 => "fuel-tank-1",
            FuelTankType::Tank2 => "fuel-tank-2",
            FuelTankType::Tank3 => "fuel-tank-3",
            FuelTankType::Tank4 => "fuel-tank-4",
            FuelTankType::Torpedo => panic!("Attempt to get torpedo fuel tank texture"),
            FuelTankType::Hub => panic!("Attempt to get hub fuel tank texture"),
        }
    }
}

impl TexturedSlot for TorpedoStorageType {
    fn texture(&self) -> &'static str {
        match self {
            TorpedoStorageType::TorpedoStorage1 => "torpedo-storage-1",
            TorpedoStorageType::TorpedoStorage2 => "torpedo-storage-2",
            TorpedoStorageType::Hub => panic!("Attempt to get hub torpedo storage texture"),
        }
    }
}

impl TexturedSlot for TorpedoLauncherType {
    fn texture(&self) -> &'static str {
        match self {
            TorpedoLauncherType::TorpedoLauncher1 => "torpedo-launcher-1",
            TorpedoLauncherType::TorpedoLauncher2 => "torpedo-launcher-2",
        }
    }
}
