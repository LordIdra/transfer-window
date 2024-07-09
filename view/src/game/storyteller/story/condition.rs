use super::story_event::StoryEvent;

pub trait Condition {
    fn met(&self, story_events: &Vec<StoryEvent>) -> bool;
}

fn story_events_contains<T: Fn(&StoryEvent) -> bool>(story_events: &Vec<StoryEvent>, condition: T) -> bool {
    story_events.iter().any(condition)
}

pub struct NoneCondition;

impl NoneCondition {
    pub fn new() -> Box<dyn Condition> {
        Box::new(Self {})
    }
}

impl Condition for NoneCondition {
    fn met(&self, _story_events: &Vec<StoryEvent>) -> bool {
        true
    }
}

pub struct ClickContinueCondition;

impl ClickContinueCondition {
    pub fn new() -> Box<dyn Condition> {
        Box::new(Self {})
    }
}

impl Condition for ClickContinueCondition {
    fn met(&self, story_events: &Vec<StoryEvent>) -> bool {
        let condition = |event: &StoryEvent| {
            matches!(*event, StoryEvent::ClickContinueEvent)
        };
        story_events_contains(story_events, condition)
    }
}