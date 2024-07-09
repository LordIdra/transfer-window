use transfer_window_model::{storage::entity_allocator::Entity, story_event::StoryEvent};

use super::{story_events_contains, ConditionCheck};

pub struct FocusCondition {
    entity: Entity,
}

impl FocusCondition {
    pub fn new(entity: Entity) -> Box<dyn ConditionCheck> {
        Box::new(Self { entity })
    }
}

impl ConditionCheck for FocusCondition {
    fn met(&self,story_events: &Vec<StoryEvent>) -> bool {
        let condition = |event: &StoryEvent| {
            if let StoryEvent::ChangeFocus(entity) = event {
                *entity == self.entity
            } else {
                false
            }
        };
        story_events_contains(story_events, condition)
    }
}

