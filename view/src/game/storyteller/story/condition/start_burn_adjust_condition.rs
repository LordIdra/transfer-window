use transfer_window_model::model::story_event::StoryEvent;

use crate::game::View;

use super::{story_events_contains, ConditionCheck};

pub struct StartBurnAdjustCondition;

impl StartBurnAdjustCondition {
    pub fn new() -> Box<dyn ConditionCheck> {
        Box::new(Self {})
    }
}

impl ConditionCheck for StartBurnAdjustCondition {
    fn met(&self, view: &View) -> bool {
        let condition = |event: &StoryEvent| {
            matches!(event, StoryEvent::StartBurnAdjust)
        };
        story_events_contains(view, condition)
    }
}
