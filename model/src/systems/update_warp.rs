use log::trace;
use serde::{Deserialize, Serialize};

use crate::api::time::{TimeStep, WARP_STOP_BEFORE_TARGET_SECONDS};
use crate::Model;

const SLOW_DOWN_AFTER_PROPORTION: f64 = 0.95;
const ADDITIONAL_MULTIPLER: f64 = 0.06;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeWarp {
    start_time: f64,
    end_time: f64,
}

impl TimeWarp {
    pub fn new(start_time: f64, end_time: f64) -> Self {
        Self {
            start_time,
            end_time: end_time - WARP_STOP_BEFORE_TARGET_SECONDS,
        }
    }

    fn compute_max_warp_speed(&self) -> f64 {
        self.end_time - self.start_time
    }

    fn compute_fraction_completed(&self, time: f64) -> f64 {
        let current_duration = time - self.start_time;
        let total_duration = self.end_time - self.start_time;
        current_duration / total_duration
    }

    pub fn compute_warp_speed(&self, time: f64) -> f64 {
        if self.compute_fraction_completed(time) < SLOW_DOWN_AFTER_PROPORTION {
            self.compute_max_warp_speed()
        } else {
            let fraction_of_last_fraction_completed = (self.compute_fraction_completed(time)
                - SLOW_DOWN_AFTER_PROPORTION)
                / (1.0 - SLOW_DOWN_AFTER_PROPORTION);
            let multiplier =
                (fraction_of_last_fraction_completed - 1.0).powi(2) + ADDITIONAL_MULTIPLER;
            multiplier * self.compute_max_warp_speed()
        }
    }
}

impl Model {
    pub(crate) fn update_warp(&mut self, dt: f64) {
        // Weird double if needed because of borrow checker
        let warp_finished = if let Some(warp) = &self.warp {
            self.time >= warp.end_time
        } else {
            return;
        };

        if warp_finished {
            trace!("Warp finished");
            self.warp = None;
            self.time_step = TimeStep::Level {
                level: 1,
                paused: false,
            };
        }

        if let Some(warp) = &self.warp {
            let mut speed = warp.compute_warp_speed(self.time);
            let final_time = self.time + speed * dt;
            if final_time > warp.end_time {
                // Oh no, we're about to overshoot
                // Calculate required warp speed to perfectly land at target point
                // Add small amount so next frame actually counts this as 'finished'
                let overshot_time = warp.end_time - self.time;
                speed = overshot_time / dt + 1.0e-3;
                trace!("Compensating for warp overshoot of {overshot_time}");
            }
            self.time_step = TimeStep::Warp {
                speed,
                paused: false,
            };
        }
    }
}
