use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use super::Model;

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
    CreateTurn(Entity),
    EnableGuidance(Entity),
    FireTorpedo(Entity),
    StartBurnAdjust,
    FireTorpedoAdjust,
    SetTarget { entity: Entity, target: Entity },
}

impl Model {
    pub fn add_story_event(&self, event: StoryEvent) {
        self.story_events.lock().unwrap().push(event);
    }
}
