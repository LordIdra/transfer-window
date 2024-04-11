use serde::{Deserialize, Serialize};

use crate::Model;

use super::time::TimeStep;

const STOP_BEFORE_TARGET_SECONDS: f64 = 5.0;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeWarp {
    start_time: f64,
    end_time: f64,
}

impl TimeWarp {
    pub fn new(start_time: f64, end_time: f64) -> Self {
        Self { start_time, end_time: end_time - STOP_BEFORE_TARGET_SECONDS }
    }

    fn get_max_warp_speed(&self) -> f64 {
        1.0 * (self.end_time - self.start_time)
    }

    fn get_fraction_completed(&self, time: f64) -> f64 {
        let current_duration = time - self.start_time;
        let total_duration = self.end_time - self.start_time;
        current_duration / total_duration
    }
    
    pub fn get_warp_speed(&self, time: f64) -> f64 {
        if self.get_fraction_completed(time) < 0.95 {
            return self.get_max_warp_speed()
        }
        let fraction_of_last_fraction_completed = (self.get_fraction_completed(time) - 0.95) * 20.0;
        let multiplier = (fraction_of_last_fraction_completed - 1.0).powi(2) + 0.06;
        multiplier * self.get_max_warp_speed()
    }
}

pub fn update(model: &mut Model, dt: f64) {
    // Weird double if needed because of borrow checker
    let warp_finished = if let Some(warp) = &model.warp {
        model.time >= warp.end_time
    } else {
        return;
    };
    if warp_finished {
        model.warp = None;
        model.time_step = TimeStep::Level { level: 1, paused: false };
    }

    if let Some(warp) = &model.warp {
        let mut speed = warp.get_warp_speed(model.time);
        let final_time = model.time + speed * dt;
        if final_time > warp.end_time {
            // Oh no, we're about to overshoot
            // Calculate required warp speed to perfectly land at target point
            // Add small amount so next frame actually counts this as 'finished'
            speed = (warp.end_time - model.time) / dt + 1.0e-3;
        }
        model.time_step = TimeStep::Warp { speed, paused: false };
    }
}