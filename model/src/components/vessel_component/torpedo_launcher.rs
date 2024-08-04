use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TorpedoLauncher {
    cooldown: f64,
    time_to_reload: f64,
}

impl TorpedoLauncher {
    pub fn new(cooldown: f64) -> TorpedoLauncher {
        let time_to_reload = 0.0;
        TorpedoLauncher { cooldown, time_to_reload,}
    }

    pub fn cooldown(&self) -> f64 {
        self.cooldown
    }
    
    pub fn time_to_reload(&self) -> f64 {
        self.time_to_reload
    }

    pub fn step_time_to_reload(&mut self, dt: f64) {
        self.time_to_reload = f64::max(0.0, self.time_to_reload - dt);
    }

    pub fn reset_time_to_reload(&mut self) {
        self.time_to_reload = self.cooldown;
    }
}