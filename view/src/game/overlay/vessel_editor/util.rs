use std::collections::BTreeMap;

use transfer_window_model::components::vessel_component::ship::{ship_slot::ShipSlotLocation, ShipClass};

pub fn compute_slot_size(vessel_class: ShipClass) -> f32 {
    match vessel_class {
        ShipClass::Frigate | ShipClass::Scout => 0.113,
    }
}

pub fn compute_slot_locations(vessel_class: ShipClass) -> BTreeMap<ShipSlotLocation, f32> {
    match vessel_class {
        ShipClass::Scout => vec![
            (ShipSlotLocation::Middle, 0.026),
            (ShipSlotLocation::Back, 0.231),
        ].into_iter().collect(),
        ShipClass::Frigate => vec![
            (ShipSlotLocation::Front, -0.142),
            (ShipSlotLocation::Middle, 0.177),
            (ShipSlotLocation::Back, 0.382),
        ].into_iter().collect(),
    }
}
