use transfer_window_model::storage::entity_allocator::Entity;
use transfer_window_model::story_event::StoryEvent;

use super::{story_events_contains, ConditionCheck};

pub struct SelectVesselCondition {
    entity: Entity,
}

impl SelectVesselCondition {
    pub fn new(entity: Entity) -> Box<dyn ConditionCheck> {
        Box::new(Self { entity })
    }
}

impl ConditionCheck for SelectVesselCondition {
    fn met(&self, story_events: &Vec<StoryEvent>) -> bool {
        let condition = |event: &StoryEvent| {
            if let StoryEvent::VesselSelected(entity) = event {
                *entity == self.entity
            } else {
                false
            }
        };
        story_events_contains(story_events, condition)
    }
}

