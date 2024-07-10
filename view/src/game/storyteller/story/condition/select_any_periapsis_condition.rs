use transfer_window_model::story_event::StoryEvent;

use super::{story_events_contains, ConditionCheck};

pub struct SelectAnyApoapsisCondition;

impl SelectAnyApoapsisCondition {
    pub fn new() -> Box<dyn ConditionCheck> {
        Box::new(Self {})
    }
}

impl ConditionCheck for SelectAnyApoapsisCondition {
    fn met(&self, story_events: &Vec<StoryEvent>) -> bool {
        let condition = |event: &StoryEvent| {
            matches!(event, StoryEvent::AnyApoapsisSelected)
        };
        story_events_contains(story_events, condition)
    }
}

