use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TorpedoLauncherType {
    TorpedoLauncher1,
    TorpedoLauncher2,
}

impl TorpedoLauncherType {
    pub fn ship_types() -> [Self; 2] {
        [
            TorpedoLauncherType::TorpedoLauncher1,
            TorpedoLauncherType::TorpedoLauncher2,
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TorpedoLauncher {
    type_: TorpedoLauncherType,
}

impl TorpedoLauncher {
    pub fn new(type_: TorpedoLauncherType) -> TorpedoLauncher {
        TorpedoLauncher {
            type_,
        }
    }
    
    pub fn type_(&self) -> TorpedoLauncherType {
        self.type_
    }
}