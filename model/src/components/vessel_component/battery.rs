use serde::{Deserialize, Serialize};

pub const FUEL_DENSITY: f64 = 1.0; // both RP-1 and LOX are very roughly 1.0kg/L

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum BatteryType {
    Battery1,
    Battery2,
    Battery3,
    BatteryHub,
}

impl BatteryType {
    pub fn ship_types() -> [Self; 3] {
        [
            BatteryType::Battery1,
            BatteryType::Battery2,
            BatteryType::Battery3,
        ]
    }

    pub fn capacity_joules(&self) -> f64 {
        match self {
            BatteryType::Battery1 => 300_000_000.0,
            BatteryType::Battery2 => 400_000_000.0,
            BatteryType::Battery3 => 600_000_000.0,
            BatteryType::BatteryHub => 1_500_000_000.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Battery {
    type_: BatteryType,
    charge_joules: f64,
}

impl Battery {
    pub fn new(type_: BatteryType) -> Self {
        let charge_joules = type_.capacity_joules() / 2.0;
        Battery { type_, charge_joules }
    }

    pub fn type_(&self) -> BatteryType {
        self.type_
    }

    pub fn capacity_joules(&self) -> f64 {
        self.type_.capacity_joules()
    }

    pub fn charge_joules(&self) -> f64 {
        self.charge_joules
    }
}