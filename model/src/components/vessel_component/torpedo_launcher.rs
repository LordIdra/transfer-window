use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TorpedoLauncherType {
    Simple,
    Enhanced,
}

impl TorpedoLauncherType {
    pub fn ship_types() -> [Self; 2] {
        [
            TorpedoLauncherType::Simple,
            TorpedoLauncherType::Enhanced,
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