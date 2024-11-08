use transfer_window_model::model::story_event::StoryEvent;

use crate::game::View;

use super::{story_events_contains, ConditionCheck};

pub struct ClickContinueCondition;

impl ClickContinueCondition {
    pub fn new() -> Box<dyn ConditionCheck> {
        Box::new(Self {})
    }
}

impl ConditionCheck for ClickContinueCondition {
    fn met(&self, view: &View) -> bool {
        let condition = |event: &StoryEvent| {
            matches!(*event, StoryEvent::ClickContinue)
        };
        story_events_contains(view, condition)
    }
}
