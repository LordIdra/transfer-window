use serde::{Deserialize, Serialize};

use self::fire_torpedo::FireTorpedoEvent;

pub mod fire_torpedo;

#[derive(Debug, Serialize, Deserialize)]
pub enum TimelineEventType {
    FireTorpedo(FireTorpedoEvent),
}

impl TimelineEventType {
    pub fn is_fire_torpedo(&self) -> bool {
        matches!(self, TimelineEventType::FireTorpedo(_))
    }

    pub fn as_fire_torpedo(&self) -> Option<&FireTorpedoEvent> {
        if let TimelineEventType::FireTorpedo(fire_torpedo) = self {
            Some(fire_torpedo)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineEvent {
    time: f64,
    type_: TimelineEventType,
}

impl TimelineEvent {
    pub fn new(time: f64, type_: TimelineEventType) -> Self {
        Self { time, type_ }
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn type_(&self) -> &TimelineEventType {
        &self.type_
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Timeline {
    events: Vec<TimelineEvent>,
}

impl Timeline {
    pub fn add(&mut self, event: TimelineEvent) {
        self.events.push(event);
    }

    pub fn fire_torpedo_events(&self) -> Vec<&TimelineEvent> {
        self.events.iter()
            .filter(|event| event.type_.is_fire_torpedo())
            .collect()
    }

    pub fn event_at_time(&self, time: f64) -> Option<&TimelineEvent> {
        for event in &self.events {
            if event.time == time {
                return Some(event);
            }
        }
        None
    }
}