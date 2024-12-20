use crate::game::View;

use super::condition::Condition;

pub struct Transition {
    to: &'static str,
    condition: Condition,
}

impl Transition {
    pub fn new(to: &'static str, condition: Condition) -> Self {
        Self { to, condition }
    }

    pub fn can_transition(&self, view: &View) -> bool {
        self.condition.met(view)
    }
    
    pub fn to(&self) -> &'static str {
        self.to
    }

    pub fn objective(&self) -> Option<&'static str> {
        self.condition.get_objective()
    }
}