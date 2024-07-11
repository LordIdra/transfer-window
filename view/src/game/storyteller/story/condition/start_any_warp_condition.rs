use transfer_window_model::story_event::StoryEvent;

use crate::game::View;

use super::{story_events_contains, ConditionCheck};

pub struct StartAnyWarpCondition;

impl StartAnyWarpCondition {
    pub fn new() -> Box<dyn ConditionCheck> {
        Box::new(Self {})
    }
}

impl ConditionCheck for StartAnyWarpCondition {
    fn met(&self, view: &View) -> bool {
        let condition = |event: &StoryEvent| {
            matches!(event, StoryEvent::WarpStarted)
        };
        story_events_contains(view, condition)
    }
}

