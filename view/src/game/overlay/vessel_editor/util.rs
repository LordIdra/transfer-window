use std::collections::BTreeMap;

use transfer_window_model::components::vessel_component::class::VesselClass;

use super::SlotType;

pub fn compute_slot_size(vessel_class: VesselClass) -> f32 {
    match vessel_class {
        VesselClass::Frigate | VesselClass::Scout => 0.08, //0.113,
        VesselClass::Torpedo | VesselClass::Hub => unreachable!(),
    }
}

pub fn compute_slot_locations(vessel_class: VesselClass) -> BTreeMap<SlotType, f32> {
    match vessel_class {
        VesselClass::Scout => vec![
            (SlotType::FuelTank, 0.026),
            (SlotType::Engine, 0.231),
        ].into_iter().collect(),
        VesselClass::Frigate => vec![
            (SlotType::TorpedoLauncher, -0.142),
            (SlotType::TorpedoStorage, 0.0),
            (SlotType::FuelTank, 0.177),
            (SlotType::Engine, 0.382),
        ].into_iter().collect(),
        VesselClass::Torpedo | VesselClass::Hub => unreachable!(),
    }
}
