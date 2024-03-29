use log::trace;
use serde::{Deserialize, Serialize};

use crate::Model;

const MIN_LEVEL: i32 = 1;
const MAX_LEVEL: i32 = 10;

#[derive(Debug, Serialize, Deserialize)]
pub enum TimeStep {
    Level { level: i32, paused: bool }, // Starts at level 1 for 1x speed
    Warp { speed: f64, paused: bool },
}

impl TimeStep {
    pub fn get_time_step(&self) -> f64 {
        if self.is_paused() {
            return 0.0;
        }

        match self {
            TimeStep::Level { level, paused: _ } => 5.0_f64.powi(*level - 1),
            TimeStep::Warp { speed, paused: _ } => *speed,
        }
    }

    pub fn toggle_paused(&mut self) {
        match self {
            TimeStep::Warp { speed: _, paused } | TimeStep::Level { level: _, paused } => *paused = !*paused,
        }
        trace!("New time state: {:?}", self);
    }

    pub fn is_paused(&self) -> bool {
        match self {
            TimeStep::Warp { speed: _, paused } | TimeStep::Level { level: _, paused } => *paused,
        }
    }

    pub fn increase_level(&mut self) {
        match self {
            TimeStep::Level { level, paused: _ } => {
                if *level < MAX_LEVEL {
                    *level += 1;
                } else {
                    trace!("Could not increase time step level past max");
                }
            }
            TimeStep::Warp { speed: _, paused: _ } => {
                trace!("Could not increase time step level because a warp is in progress");
            },
        }
        trace!("New time state: {:?}", self);
    }

    pub fn decrease_level(&mut self) {
        match self {
            TimeStep::Level { level, paused: _ } => {
                if *level > MIN_LEVEL {
                    *level -= 1;
                } else {
                    trace!("Could not decrease time step level past min");
                }
            }
            TimeStep::Warp { speed: _, paused: _ } => {
                trace!("Could not decrease time step level because a warp is in progress");
            },
        }
        trace!("New time state: {:?}", self);
    }
}

pub fn update(model: &mut Model, dt: f64) {
    model.time += dt * model.time_step.get_time_step();
}