use click_continue_condition::ClickContinueCondition;
use focus_condition::FocusCondition;
use none_condition::NoneCondition;
use pause_condition::PauseCondition;
use time_condition::TimeCondition;
use transfer_window_model::{storage::entity_allocator::Entity, story_event::StoryEvent};

pub mod click_continue_condition;
pub mod focus_condition;
pub mod none_condition;
mod pause_condition;
pub mod time_condition;

pub struct Condition {
    check: Box<dyn ConditionCheck>,
    objective: Option<&'static str>,
}

impl Condition {
    pub fn click_continue() -> Self {
        Self { check: ClickContinueCondition::new(), objective: None }
    }

    pub fn focus(entity: Entity) -> Self {
        Self { check: FocusCondition::new(entity), objective: None }
    }

    pub fn none() -> Self {
        Self { check: NoneCondition::new(), objective: None }
    }

    pub fn pause() -> Self {
        Self { check: PauseCondition::new(), objective: None }
    }

    pub fn time(time: f64) -> Self {
        Self { check: TimeCondition::new(time), objective: None }
    }

    pub fn objective(mut self, objective: &'static str) -> Self {
        self.objective = Some(objective);
        self
    }

    pub(super) fn met(&self, story_events: &Vec<StoryEvent>) -> bool {
        self.check.met(story_events)
    }

    pub(super) fn get_objective(&self) -> Option<&'static str> {
        self.objective
    }
}

pub trait ConditionCheck {
    fn met(&self, story_events: &Vec<StoryEvent>) -> bool;
}

fn story_events_contains<T: Fn(&StoryEvent) -> bool>(story_events: &Vec<StoryEvent>, condition: T) -> bool {
    story_events.iter().any(condition)
}
