use ecolor::Rgba;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AtmosphereComponent {
    color: Rgba,
    density: f64,
    height: f64,
    falloff: f64,
}

impl AtmosphereComponent {
    pub fn new(color: Rgba, density: f64, height: f64, falloff: f64) -> Self {
        Self { color, density, height, falloff }
    }

    pub fn color(&self) -> Rgba {
        self.color
    }

    pub fn density(&self) -> f64 {
        self.density
    }

    pub fn height(&self) -> f64 {
        self.height
    }
    
    pub fn falloff(&self) -> f64 {
        self.falloff
    }
}