use nalgebra_glm::DVec2;
use transfer_window_model::storage::{entity_allocator::Entity, entity_builder::EntityBuilder};

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
    DebugAddEntity { entity_builder: EntityBuilder },
}