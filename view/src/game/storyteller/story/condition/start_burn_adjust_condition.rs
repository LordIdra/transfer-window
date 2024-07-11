use transfer_window_model::story_event::StoryEvent;

use super::{story_events_contains, ConditionCheck};

pub struct StartBurnAdjustCondition;

impl StartBurnAdjustCondition {
    pub fn new() -> Box<dyn ConditionCheck> {
        Box::new(Self {})
    }
}

impl ConditionCheck for StartBurnAdjustCondition {
    fn met(&self, story_events: &Vec<StoryEvent>) -> bool {
        let condition = |event: &StoryEvent| {
            matches!(event, StoryEvent::StartBurnAdjust)
        };
        story_events_contains(story_events, condition)
    }
}
