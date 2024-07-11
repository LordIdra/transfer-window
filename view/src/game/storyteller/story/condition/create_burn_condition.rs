use transfer_window_model::storage::entity_allocator::Entity;
use transfer_window_model::story_event::StoryEvent;

use super::{story_events_contains, ConditionCheck};

pub struct CreateBurnCondition {
    entity: Entity,
}

impl CreateBurnCondition {
    pub fn new(entity: Entity) -> Box<dyn ConditionCheck> {
        Box::new(Self { entity })
    }
}

impl ConditionCheck for CreateBurnCondition {
    fn met(&self, story_events: &Vec<StoryEvent>) -> bool {
        let condition = |event: &StoryEvent| {
            if let StoryEvent::CreateBurn(entity) = event {
                *entity == self.entity
            } else {
                false
            }
        };
        story_events_contains(story_events, condition)
    }
}

