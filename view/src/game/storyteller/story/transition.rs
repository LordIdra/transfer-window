use super::{condition::Condition, story_event::StoryEvent};

pub struct Transition {
    to: &'static str,
    condition: Box<dyn Condition>,
}

impl Transition {
    pub fn new(to: &'static str, condition: Box<dyn Condition>) -> Self {
        Self { to, condition }
    }

    pub fn try_transition(&self, events: &Vec<StoryEvent>) -> Option<&'static str> {
        if self.condition.met(events) {
            Some(self.to)
        } else {
            None
        }
    }
    
    #[cfg(test)]
    pub fn to(&self) -> &str {
        self.to
    }
}