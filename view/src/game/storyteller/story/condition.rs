use click_continue_condition::ClickContinueCondition;
use create_burn_condition::CreateBurnCondition;
use enable_guidance_condition::EnableGuidanceCondition;
use fire_torpedo_adjust_condition::FireTorpedoAdjustCondition;
use fire_torpedo_condition::FireTorpedoCondition;
use first_closest_approach_condition::FirstClosestApproachCondition;
use focus_condition::FocusCondition;
use get_intercept::GetInterceptCondition;
use last_orbit_apoapsis_condition::LastOrbitApoapsis;
use last_orbit_circular_condition::LastOrbitCircular;
use none_condition::NoneCondition;
use pause_condition::PauseCondition;
use select_any_orbit_point_condition::SelectAnyOrbitPointCondition;
use select_any_periapsis_condition::SelectAnyApoapsisCondition;
use select_vessel_condition::SelectVesselCondition;
use set_target::SetTargetCondition;
use start_any_warp_condition::StartAnyWarpCondition;
use start_burn_adjust_condition::StartBurnAdjustCondition;
use time_condition::TimeCondition;
use transfer_window_model::{model::story_event::StoryEvent, storage::entity_allocator::Entity};

use crate::game::View;

mod click_continue_condition;
mod create_burn_condition;
mod enable_guidance_condition;
mod fire_torpedo_condition;
mod first_closest_approach_condition;
mod focus_condition;
mod get_intercept;
mod last_orbit_apoapsis_condition;
mod last_orbit_circular_condition;
mod none_condition;
mod pause_condition;
mod select_any_orbit_point_condition;
mod select_any_periapsis_condition;
mod select_vessel_condition;
mod set_target;
mod start_any_warp_condition;
mod start_burn_adjust_condition;
mod fire_torpedo_adjust_condition;
mod time_condition;

pub struct Condition {
    check: Box<dyn ConditionCheck>,
    objective: Option<&'static str>,
}

impl Condition {
    pub fn click_continue() -> Self {
        Self { check: ClickContinueCondition::new(), objective: None }
    }

    pub fn create_burn(entity: Entity) -> Self {
        Self { check: CreateBurnCondition::new(entity), objective: None }
    }

    pub fn enable_guidance(entity: Entity) -> Self {
        Self { check: EnableGuidanceCondition::new(entity), objective: None }
    }

    pub fn fire_torpedo_adjust() -> Self {
        Self { check: FireTorpedoAdjustCondition::new(), objective: None }
    }

    pub fn fire_torpedo(entity: Entity) -> Self {
        Self { check: FireTorpedoCondition::new(entity), objective: None }
    }

    pub fn first_closest_approach(entity: Entity, max_distance: f64) -> Self {
        Self { check: FirstClosestApproachCondition::new(entity, max_distance), objective: None }
    }

    pub fn focus(entity: Entity) -> Self {
        Self { check: FocusCondition::new(entity), objective: None }
    }

    pub fn get_intercept(entity: Entity) -> Self {
        Self { check: GetInterceptCondition::new(entity), objective: None }
    }

    pub fn last_orbit_apoapsis(entity: Entity, min: f64, max: f64) -> Self {
        Self { check: LastOrbitApoapsis::new(entity, min, max), objective: None }
    }

    pub fn last_orbit_circular(entity: Entity, min: f64, max: f64) -> Self {
        Self { check: LastOrbitCircular::new(entity, min, max), objective: None }
    }

    pub fn none() -> Self {
        Self { check: NoneCondition::new(), objective: None }
    }

    pub fn pause() -> Self {
        Self { check: PauseCondition::new(), objective: None }
    }

    pub fn select_any_orbit_point(entity: Entity) -> Self {
        Self { check: SelectAnyOrbitPointCondition::new(entity), objective: None }
    }

    pub fn select_any_apoapsis(entity: Entity) -> Self {
        Self { check: SelectAnyApoapsisCondition::new(entity), objective: None }
    }

    pub fn select_vessel(entity: Entity) -> Self {
        Self { check: SelectVesselCondition::new(entity), objective: None }
    }

    pub fn set_target(entity: Entity, target: Entity) -> Self {
        Self { check: SetTargetCondition::new(entity, target), objective: None }
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

    pub(super) fn met(&self, view: &View) -> bool {
        self.check.met(view)
    }

    pub(super) fn get_objective(&self) -> Option<&'static str> {
        self.objective
    }
}

pub trait ConditionCheck {
    fn met(&self, view: &View) -> bool;
}

fn story_events_contains<T: Fn(&StoryEvent) -> bool>(view: &View, condition: T) -> bool {
    view.previous_story_events.iter().any(condition)
}
