use transfer_window_model::story_event::StoryEvent;

use super::{story_events_contains, ConditionCheck};

pub struct PauseCondition;

impl PauseCondition {
    pub fn new() -> Box<dyn ConditionCheck> {
        Box::new(Self {})
    }
}

impl ConditionCheck for PauseCondition {
    fn met(&self, story_events: &Vec<StoryEvent>) -> bool {
        let condition = |event: &StoryEvent| matches!(event, StoryEvent::Paused);
        story_events_contains(story_events, condition)
    }
}
