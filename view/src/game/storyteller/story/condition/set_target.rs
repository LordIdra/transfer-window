use transfer_window_model::storage::entity_allocator::Entity;
use transfer_window_model::model::story_event::StoryEvent;

use crate::game::View;

use super::{story_events_contains, ConditionCheck};

pub struct SetTargetCondition {
    entity: Entity,
    target: Entity,
}

impl SetTargetCondition {
    pub fn new(entity: Entity, target: Entity) -> Box<dyn ConditionCheck> {
        Box::new(Self { entity, target })
    }
}

impl ConditionCheck for SetTargetCondition {
    fn met(&self, view: &View) -> bool {
        let condition = |event: &StoryEvent| {
            if let StoryEvent::SetTarget { entity, target } = event {
                *entity == self.entity && *target == self.target
            } else {
                false
            }
        };
        story_events_contains(view, condition)
    }
}
