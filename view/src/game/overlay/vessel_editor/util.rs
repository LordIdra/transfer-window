use std::collections::BTreeMap;

use eframe::egui::{vec2, Vec2};
use transfer_window_model::components::vessel_component::class::VesselClass;

use super::SlotType;

pub const SLOT_SIZE: f32 = 0.06;

#[allow(clippy::match_same_arms)]
pub fn compute_slot_locations(vessel_class: VesselClass) -> BTreeMap<SlotType, Vec2> {
    match vessel_class {
        VesselClass::Scout1 => vec![
            (SlotType::FuelTank, vec2(0.091, 0.0)),
            (SlotType::Engine, vec2(0.199, 0.0)),
        ].into_iter().collect(),
        VesselClass::Scout2 => vec![
            (SlotType::FuelTank, vec2(0.134, 0.0)),
            (SlotType::Engine, vec2(0.242, 0.0)),
        ].into_iter().collect(),
        VesselClass::Frigate1 => vec![
            (SlotType::FuelTank, vec2(0.126, 0.0)),
            (SlotType::Engine, vec2(0.232, 0.0)),
            (SlotType::TorpedoStorage, vec2(-0.085, 0.0)),
            (SlotType::TorpedoLauncher, vec2(-0.192, 0.0)),
        ].into_iter().collect(),
        VesselClass::Frigate2 => vec![
            (SlotType::FuelTank, vec2(0.126, 0.0)),
            (SlotType::Engine, vec2(0.232, 0.0)),
            (SlotType::TorpedoStorage, vec2(-0.085, 0.0)),
            (SlotType::TorpedoLauncher, vec2(-0.192, 0.0)),
        ].into_iter().collect(),
        VesselClass::Torpedo | VesselClass::Hub => unreachable!(),
    }
}
