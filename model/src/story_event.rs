use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StoryEvent {
    ClickContinue,
    NewTime(f64),
    ChangeFocus(Entity),
    Paused,
    ApoapsisSelected(Entity),
    OrbitPointSelected(Entity),
    VesselSelected(Entity),
    WarpStarted,
    CreateBurn(Entity),
    EnableGuidance(Entity),
    FireTorpedo(Entity),
    StartBurnAdjust,
    FireTorpedoAdjust,
    SetTarget { entity: Entity, target: Entity },
}
