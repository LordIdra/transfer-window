use crate::game::events::{ModelEvent, ViewEvent};

pub mod close_dialogue_action;
pub mod create_vessel_action;
pub mod delete_vessel_action;
pub mod finish_level_action;
pub mod set_focus_action;
pub mod set_time_step_action;
pub mod show_dialogue_action;

pub trait Action {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>);
}
