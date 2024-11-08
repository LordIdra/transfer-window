use transfer_window_model::storage::entity_allocator::Entity;
use transfer_window_model::model::story_event::StoryEvent;

use crate::game::View;

use super::{story_events_contains, ConditionCheck};

pub struct SelectAnyOrbitPointCondition {
    entity: Entity,
}

impl SelectAnyOrbitPointCondition {
    pub fn new(entity: Entity) -> Box<dyn ConditionCheck> {
        Box::new(Self { entity })
    }
}

impl ConditionCheck for SelectAnyOrbitPointCondition {
    fn met(&self, view: &View) -> bool {
        let condition = |event: &StoryEvent| {
            if let StoryEvent::OrbitPointSelected(entity) = event {
                *entity == self.entity
            } else {
                false
            }
        };
        story_events_contains(view, condition)
    }
}

