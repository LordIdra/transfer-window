use transfer_window_model::story_event::StoryEvent;

use super::{story_events_contains, ConditionCheck};

pub struct SelectAnyOrbitPointCondition;

impl SelectAnyOrbitPointCondition {
    pub fn new() -> Box<dyn ConditionCheck> {
        Box::new(Self {})
    }
}

impl ConditionCheck for SelectAnyOrbitPointCondition {
    fn met(&self, story_events: &Vec<StoryEvent>) -> bool {
        let condition = |event: &StoryEvent| {
            matches!(event, StoryEvent::AnyOrbitPointSelected)
        };
        story_events_contains(story_events, condition)
    }
}

