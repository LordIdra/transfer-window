#[allow(clippy::wildcard_imports)]
use handlers::*;
use log::debug;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::vessel_component::system_slot::{Slot, SlotLocation}, storage::entity_allocator::Entity};

use super::View;

mod handlers;

#[derive(Debug)]
pub enum Event {
    #[allow(unused)]
    SaveGame { name: String },
    TogglePaused,
    IncreaseTimeStepLevel,
    DecreaseTimeStepLevel,
    StartWarp { end_time: f64 },
    CreateBurn { entity: Entity, time: f64 },
    AdjustBurn { entity: Entity, time: f64, amount: DVec2 },
    SetTarget { entity: Entity, target: Option<Entity> },
    SetSlot { entity: Entity, slot_location: SlotLocation, slot: Slot },
    CreateFireTorpedo { entity: Entity, slot_location: SlotLocation, time: f64 },
    AdjustFireTorpedo { entity: Entity, time: f64, amount: DVec2 },
    CreateGuidance { entity: Entity, time: f64, },
    CancelLastTimelineEvent { entity: Entity },
    CancelCurrentSegment { entity: Entity },
}

pub fn update(view: &mut View) {
    while let Some(event) = view.events.pop() {
        debug!("Handling event {:?}", event);
        match event {
            Event::SaveGame { name } => save_game(view, name.as_str()),
            Event::TogglePaused => toggle_paused(view),
            Event::IncreaseTimeStepLevel => increase_time_step_level(view),
            Event::DecreaseTimeStepLevel => decrease_time_step_level(view),
            Event::StartWarp { end_time } => start_warp(view, end_time),
            Event::CreateBurn { entity, time } => create_burn(view, entity, time),
            Event::AdjustBurn { entity, time, amount } => adjust_burn(view, entity, time, amount),
            Event::SetTarget { entity, target } => set_target(view, entity, target),
            Event::SetSlot { entity, slot_location, slot } => set_slot(view, entity, slot_location, slot),
            Event::CreateFireTorpedo { entity, slot_location, time } => create_fire_torpedo(view, entity, slot_location, time),
            Event::AdjustFireTorpedo { entity, time, amount } => adjust_fire_torpedo(view, entity, time, amount),
            Event::CancelLastTimelineEvent { entity } => cancel_last_event(view, entity),
            Event::CreateGuidance { entity, time } => enable_torpedo_guidance(view, entity, time),
            Event::CancelCurrentSegment { entity } => cancel_current_segment(view, entity),
        }
    }
}