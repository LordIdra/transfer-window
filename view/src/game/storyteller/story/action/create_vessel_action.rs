use transfer_window_model::api::builder::VesselBuilder;

use crate::game::events::{ModelEvent, ViewEvent};

use super::Action;

pub struct CreateVesselAction{
    vessel_builder: VesselBuilder,
}

impl CreateVesselAction {
    pub fn new(vessel_builder: VesselBuilder) -> Box<dyn Action> {
        Box::new(Self { vessel_builder })
    }
}

impl Action for CreateVesselAction {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let event = ModelEvent::BuildVessel { vessel_builder: self.vessel_builder.clone() };
        (vec![event], vec![])
    }
}