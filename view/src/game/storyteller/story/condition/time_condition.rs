use transfer_window_model::story_event::StoryEvent;

use crate::game::View;

use super::{story_events_contains, ConditionCheck};

pub struct TimeCondition{
    time: f64,
}

impl TimeCondition {
    pub fn new(time: f64) -> Box<dyn ConditionCheck> {
        Box::new(Self { time })
    }
}

impl ConditionCheck for TimeCondition {
    fn met(&self, view: &View) -> bool {
        let condition = |event: &StoryEvent| {
            if let StoryEvent::NewTime(time) = event {
                *time >= self.time
            } else {
                false
            }
        };
        story_events_contains(view, condition)
    }
}

