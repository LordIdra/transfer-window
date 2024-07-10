use log::trace;
use serde::{Deserialize, Serialize};

use crate::{story_event::StoryEvent, systems::update_warp::TimeWarp, Model};

pub const WARP_STOP_BEFORE_TARGET_SECONDS: f64 = 5.0;

pub const TIME_STEP_LEVELS: [f64; 13] = [
    1.0, 5.0, 15.0,  // 1s, 5s, 15s
    60.0, 300.0, 900.0, // 1m, 5m, 15m
    3600.0, 21600.0, // 1h, 6h
    86400.0, 432_000.0, 2_160_000.0, 8_640_000.0, // 1d, 5d, 25d, 100d
    31_536_000.0 // 1y
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeStep {
    Level { level: i32, paused: bool }, // Starts at level 1 for 1x speed
    Warp { speed: f64, paused: bool },
}

impl TimeStep {
    pub fn time_step(&self) -> f64 {
        if self.paused() {
            return 0.0;
        }

        match self {
            TimeStep::Level { level, paused: _ } => TIME_STEP_LEVELS[(*level - 1) as usize],
            TimeStep::Warp { speed, paused: _ } => *speed,
        }
    }

    pub fn paused(&self) -> bool {
        match self {
            TimeStep::Warp { speed: _, paused } | TimeStep::Level { level: _, paused } => *paused,
        }
    }

    pub(crate) fn toggle_paused(&mut self) {
        match self {
            TimeStep::Warp { speed: _, paused } | TimeStep::Level { level: _, paused } => *paused = !*paused,
        }
        trace!("New time state: {:?}", self);
    }

    pub(crate) fn increase_level(&mut self) {
        match self {
            TimeStep::Level { level, paused: _ } => {
                if *level < TIME_STEP_LEVELS.len() as i32 {
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

    pub(crate) fn decrease_level(&mut self) {
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
    pub fn toggle_paused(&mut self) {
        self.time_step.toggle_paused();
        if self.time_step.paused() {
            self.add_story_event(StoryEvent::Paused);
        }
    }

    pub fn increase_time_step_level(&mut self) {
        self.time_step.increase_level();
    }

    pub fn decrease_time_step_level(&mut self) {
        self.time_step.decrease_level();
    }

    pub fn set_time_step(&mut self, time_step: TimeStep) {
        self.time_step = time_step;
    }

    pub fn start_warp(&mut self, end_time: f64) {
        self.add_story_event(StoryEvent::WarpStarted);
        self.warp = Some(TimeWarp::new(self.time, end_time));
    }

    pub fn time_step(&self) -> &TimeStep {
        &self.time_step
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn can_warp_to(&self, time: f64) -> bool {
        (time - self.time()).abs() > WARP_STOP_BEFORE_TARGET_SECONDS
    }

    pub fn warp(&self) -> Option<&TimeWarp> {
        self.warp.as_ref()
    }
}