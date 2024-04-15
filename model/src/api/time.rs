use crate::{systems::{time::TimeStep, warp_update_system::TimeWarp}, Model};

impl Model {
    pub fn toggle_paused(&mut self) {
        self.time_step.toggle_paused();
    }

    pub fn increase_time_step_level(&mut self) {
        self.time_step.increase_level();
    }

    pub fn decrease_time_step_level(&mut self) {
        self.time_step.decrease_level();
    }

    pub fn start_warp(&mut self, end_time: f64) {
        self.warp = Some(TimeWarp::new(self.time, end_time));
    }

    pub fn get_time_step(&self) -> &TimeStep {
        &self.time_step
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }
}