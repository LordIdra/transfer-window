use nalgebra_glm::DVec2;
use transfer_window_model::{components::vessel_component::system_slot::{Slot, SlotLocation}, storage::entity_allocator::Entity};

#[derive(Debug)]
pub enum Event {
    NewGame,
    Quit,
    SaveGame { name: String },
    LoadGame { name: String },
    TogglePaused,
    IncreaseTimeStepLevel,
    DecreaseTimeStepLevel,
    StartWarp { end_time: f64 },
    CreateBurn { entity: Entity, time: f64 },
    AdjustBurn { entity: Entity, time: f64, amount: DVec2 },
    Destroy { entity: Entity },
    SetTarget { entity: Entity, target: Option<Entity> },
    SetSlot { entity: Entity, slot_location: SlotLocation, slot: Slot },
    CreateFireTorpedo { entity: Entity, slot_location: SlotLocation, time: f64 },
    AdjustFireTorpedo { entity: Entity, time: f64, amount: DVec2 },
    CreateGuidance { entity: Entity, time: f64, },
    CancelLastTimelineEvent { entity: Entity },
}