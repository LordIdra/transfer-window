use transfer_window_model::storage::entity_builder::EntityBuilder;

#[derive(Debug)]
pub enum Event {
    NewGame,
    Quit,
    SaveGame { name: String },
    LoadGame { name: String },
    TogglePaused,
    IncreaseTimeStepLevel,
    DecreaseTimeStepLevel,
    DebugAddEntity { entity_builder: EntityBuilder },
}