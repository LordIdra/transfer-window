use std::collections::BTreeMap;

use transfer_window_model::components::vessel_component::{system_slot::SlotLocation, VesselClass};

pub fn compute_slot_size(vessel_class: VesselClass) -> f32 {
    match vessel_class {
        VesselClass::Light => 0.113,
        VesselClass::Torpedo(_) => unreachable!(),
    }
}

pub fn compute_slot_locations(vessel_class: VesselClass) -> BTreeMap<SlotLocation, f32> {
    match vessel_class {
        VesselClass::Torpedo(_) => unreachable!(),
        VesselClass::Light => vec![
            (SlotLocation::Front, -0.142),
            (SlotLocation::Middle, 0.177),
            (SlotLocation::Back, 0.382),
        ].into_iter().collect(),
    }
}
