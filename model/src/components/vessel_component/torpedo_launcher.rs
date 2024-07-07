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

    pub fn cooldown(&self) -> f64 {
        match self {
            TorpedoLauncherType::TorpedoLauncher1 => 6.0 * 60.0 * 60.0,
            TorpedoLauncherType::TorpedoLauncher2 => 2.0 * 60.0 * 60.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TorpedoLauncher {
    type_: TorpedoLauncherType,
    time_to_reload: f64,
}

impl TorpedoLauncher {
    pub fn new(type_: TorpedoLauncherType) -> TorpedoLauncher {
        let time_to_reload = 0.0;
        TorpedoLauncher {
            type_,
            time_to_reload,
        }
    }
    
    pub fn type_(&self) -> TorpedoLauncherType {
        self.type_
    }
    
    pub fn time_to_reload(&self) -> f64 {
        self.time_to_reload
    }

    pub fn step_time_to_reload(&mut self, dt: f64) {
        self.time_to_reload = f64::max(0.0, self.time_to_reload - dt);
    }

    pub fn reset_time_to_reload(&mut self) {
        self.time_to_reload = self.type_.cooldown();
    }
}