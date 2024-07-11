use click_continue_condition::ClickContinueCondition;
use create_burn_condition::CreateBurnCondition;
use enable_guidance_condition::EnableGuidanceCondition;
use fire_torpedo_condition::FireTorpedoCondition;
use focus_condition::FocusCondition;
use none_condition::NoneCondition;
use pause_condition::PauseCondition;
use select_any_orbit_point_condition::SelectAnyOrbitPointCondition;
use select_any_periapsis_condition::SelectAnyApoapsisCondition;
use select_vessel_condition::SelectVesselCondition;
use start_any_warp_condition::StartAnyWarpCondition;
use start_burn_adjust_condition::StartBurnAdjustCondition;
use time_condition::TimeCondition;
use transfer_window_model::{storage::entity_allocator::Entity, story_event::StoryEvent};

mod click_continue_condition;
mod create_burn_condition;
mod enable_guidance_condition;
mod fire_torpedo_condition;
mod focus_condition;
mod none_condition;
mod pause_condition;
mod select_any_orbit_point_condition;
mod select_any_periapsis_condition;
mod select_vessel_condition;
mod start_any_warp_condition;
mod start_burn_adjust_condition;
mod time_condition;

pub struct Condition {
    check: Box<dyn ConditionCheck>,
    objective: Option<&'static str>,
}

impl Condition {
    pub fn click_continue() -> Self {
        Self { check: ClickContinueCondition::new(), objective: None }
    }

    pub fn create_burn_condition(entity: Entity) -> Self {
        Self { check: CreateBurnCondition::new(entity), objective: None }
    }

    pub fn enable_guidance_condition(entity: Entity) -> Self {
        Self { check: EnableGuidanceCondition::new(entity), objective: None }
    }

    pub fn fire_torpedo_condition(entity: Entity) -> Self {
        Self { check: FireTorpedoCondition::new(entity), objective: None }
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

    pub fn select_any_orbit_point() -> Self {
        Self { check: SelectAnyOrbitPointCondition::new(), objective: None }
    }

    pub fn select_any_apoapsis() -> Self {
        Self { check: SelectAnyApoapsisCondition::new(), objective: None }
    }

    pub fn select_vessel(entity: Entity) -> Self {
        Self { check: SelectVesselCondition::new(entity), objective: None }
    }

    pub fn start_any_warp() -> Self {
        Self { check: StartAnyWarpCondition::new(), objective: None }
    }

    pub fn start_burn_adjust() -> Self {
        Self { check: StartBurnAdjustCondition::new(), objective: None }
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
