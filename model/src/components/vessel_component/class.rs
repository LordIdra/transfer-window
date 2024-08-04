use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum VesselClass {
    Torpedo,
    Station,
    Scout1,
    Frigate1,
}

impl VesselClass {
    pub fn name(&self) -> &'static str {
        match self {
            VesselClass::Torpedo => "Torpedo",
            VesselClass::Station => "Hub",
            VesselClass::Scout1 => "Scout I",
            VesselClass::Frigate1 => "Frigate I",
        }
    }

    pub fn is_torpedo(&self) -> bool {
        matches!(self, VesselClass::Torpedo)
    }
}
