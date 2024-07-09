use transfer_window_model::story_event::StoryEvent;

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
    fn met(&self, story_events: &Vec<StoryEvent>) -> bool {
        let condition = |event: &StoryEvent| {
            if let StoryEvent::NewTime(time) = event {
                *time >= self.time
            } else {
                false
            }
        };
        story_events_contains(story_events, condition)
    }
}

