use transfer_window_model::api::time::TimeStep;

use super::Action;
use crate::game::events::{ModelEvent, ViewEvent};

pub struct SetTimeStepAction {
    time_step: TimeStep,
}

impl SetTimeStepAction {
    pub fn new(time_step: TimeStep) -> Box<dyn Action> {
        Box::new(Self { time_step })
    }
}

impl Action for SetTimeStepAction {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let event = ModelEvent::SetTimeStep {
            time_step: self.time_step.clone(),
        };
        (vec![event], vec![])
    }
}
