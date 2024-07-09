use transfer_window_model::story_event::StoryEvent;

use super::ConditionCheck;

pub struct NoneCondition;

impl NoneCondition {
    pub fn new() -> Box<dyn ConditionCheck> {
        Box::new(Self {})
    }
}

impl ConditionCheck for NoneCondition {
    fn met(&self, _story_events: &Vec<StoryEvent>) -> bool {
        true
    }
}

