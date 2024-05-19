use log::trace;
use serde::{Deserialize, Serialize};

use crate::Model;

const LEVELS: [f64; 13] = [
    1.0, 5.0, 15.0,  // 1s, 5s, 15s
    60.0, 300.0, 900.0, // 1m, 5m, 15m
    3600.0, 21600.0, // 1h, 6h
    86400.0, 432_000.0, 2_160_000.0, 8_640_000.0, // 1d, 5d, 25d, 100d
    31_536_000.0 // 1y
];

#[derive(Debug, Serialize, Deserialize)]
pub enum TimeStep {
    Level { level: i32, paused: bool }, // Starts at level 1 for 1x speed
    Warp { speed: f64, paused: bool },
}

impl TimeStep {
    pub fn time_step(&self) -> f64 {
        if self.is_paused() {
            return 0.0;
        }

        match self {
            TimeStep::Level { level, paused: _ } => LEVELS[(*level - 1) as usize],
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
                if *level < LEVELS.len() as i32 {
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
                if *level > 1 {
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

impl Model {
    pub(crate) fn update_time(&mut self, dt: f64) {
        self.time += dt * self.time_step.time_step();
    }
}