use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use self::fire_torpedo::FireTorpedoEvent;

pub mod fire_torpedo;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn as_fire_torpedo_mut(&mut self) -> Option<&mut FireTorpedoEvent> {
        if let TimelineEventType::FireTorpedo(fire_torpedo) = self {
            Some(fire_torpedo)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn type_mut(&mut self) -> &mut TimelineEventType {
        &mut self.type_
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Timeline {
    events: VecDeque<TimelineEvent>,
}

impl Timeline {
    pub fn add(&mut self, event: TimelineEvent) {
        self.events.push_back(event);
    }

    pub fn fire_torpedo_events(&self) -> Vec<&TimelineEvent> {
        self.events.iter()
            .filter(|event| event.type_.is_fire_torpedo())
            .collect()
    }

    pub fn event_at_time(&self, time: f64) -> Option<&TimelineEvent> {
        self.events.iter().find(|event| event.time == time)
    }

    pub fn event_at_time_mut(&mut self, time: f64) -> Option<&mut TimelineEvent> {
        self.events.iter_mut().find(|event| event.time == time)
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn pop_events_before(&mut self, time: f64) -> Vec<TimelineEvent> {
        let mut events = vec![];
        while self.events.front().is_some_and(|event| event.time < time) {
            events.push(self.events.pop_front().unwrap());
        }
        events
    }
}