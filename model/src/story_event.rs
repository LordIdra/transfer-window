use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StoryEvent {
    ClickContinue,
    NewTime(f64),
    ChangeFocus(Entity),
    Paused,
    AnyApoapsisSelected,
    AnyOrbitPointSelected,
    VesselSelected(Entity),
    WarpStarted,
    CreateBurn(Entity),
    EnableGuidance(Entity),
    FireTorpedo(Entity),
    StartBurnAdjust,
}