use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum GeneratorType {
    SolarPanel1,
    SolarPanel2,
    FissionReactor,
    HubGenerator,
}

impl GeneratorType {
    pub fn ship_types() -> [Self; 3] {
        [
            GeneratorType::SolarPanel1,
            GeneratorType::SolarPanel2,
            GeneratorType::FissionReactor,
        ]
    }

    pub fn power_output_watts(&self) -> f64 {
        match self {
            GeneratorType::SolarPanel1 => 15_000.0,
            GeneratorType::SolarPanel2 => 25_000.0,
            GeneratorType::FissionReactor => 50_000.0,
            GeneratorType::HubGenerator => 65_000.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Generator {
    type_: GeneratorType,
}

impl Generator {
    pub fn new(type_: GeneratorType) -> Generator {
        Generator {
            type_,
        }
    }
    
    pub fn type_(&self) -> GeneratorType {
        self.type_
    }

    pub fn power_output_watts(&self) -> f64 {
        self.type_.power_output_watts()
    }
}