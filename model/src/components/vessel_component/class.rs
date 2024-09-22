use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum VesselClass {
    Torpedo,
    Station,
    Scout1,
    Frigate1,
    /// Ship with a high thrust, high fuel consumption, and lots of facilities for testing
    TestShip,
}

impl VesselClass {
    pub fn name(&self) -> &'static str {
        match self {
            VesselClass::Torpedo => "Torpedo",
            VesselClass::Station => "Hub",
            VesselClass::Scout1 => "Scout I",
            VesselClass::Frigate1 => "Frigate I",
            VesselClass::TestShip => "Test Ship"
        }
    }

    pub fn is_torpedo(&self) -> bool {
        matches!(self, VesselClass::Torpedo)
    }
}
