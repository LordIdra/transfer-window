use transfer_window_model::storage::entity_allocator::Entity;
use transfer_window_model::story_event::StoryEvent;

use crate::game::View;

use super::{story_events_contains, ConditionCheck};

pub struct FireTorpedoCondition {
    entity: Entity,
}

impl FireTorpedoCondition {
    pub fn new(entity: Entity) -> Box<dyn ConditionCheck> {
        Box::new(Self { entity })
    }
}

impl ConditionCheck for FireTorpedoCondition {
    fn met(&self, view: &View) -> bool {
        let condition = |event: &StoryEvent| {
            if let StoryEvent::FireTorpedo(entity) = event {
                *entity == self.entity
            } else {
                false
            }
        };
        story_events_contains(view, condition)
    }
}

