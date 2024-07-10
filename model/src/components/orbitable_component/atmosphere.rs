use ecolor::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Atmosphere {
    color: Color32,
    density: f64,
    height: f64,
    falloff: f64,
}

impl Atmosphere {
    
    pub fn new(color: Color32, density: f64, height: f64, falloff: f64) -> Self {
        Self { color, density, height, falloff }
    }
    
    /// Convenience constructor
    pub fn new_some(color32: Color32, density: f64, height: f64, falloff: f64) -> Option<Self> {
        Some(Self::new(color32, density, height, falloff))
    }

    pub fn color(&self) -> Color32 {
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