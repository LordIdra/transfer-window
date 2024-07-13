use transfer_window_model::story_event::StoryEvent;

use crate::game::View;

use super::{story_events_contains, ConditionCheck};

pub struct FireTorpedoAdjustCondition;

impl FireTorpedoAdjustCondition {
    pub fn new() -> Box<dyn ConditionCheck> {
        Box::new(Self {})
    }
}

impl ConditionCheck for FireTorpedoAdjustCondition {
    fn met(&self, view: &View) -> bool {
        let condition = |event: &StoryEvent| {
            matches!(event, StoryEvent::FireTorpedoAdjust)
        };
        story_events_contains(view, condition)
    }
}
