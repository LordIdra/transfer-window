use std::collections::VecDeque;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::Model;

use self::{start_burn::BurnEvent, fire_torpedo::FireTorpedoEvent};

use super::system_slot::SlotLocation;

pub mod start_burn;
pub mod fire_torpedo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TimelineEventType {
    FireTorpedo(FireTorpedoEvent),
    Burn(BurnEvent),
}

impl TimelineEventType {
    pub fn execute(&self, model: &mut Model) {
        match self {
            TimelineEventType::FireTorpedo(event) => event.execute(model),
            TimelineEventType::Burn(event) => event.execute(model),
        }
    }

    pub fn cancel(&self, model: &mut Model) {
        match self {
            TimelineEventType::FireTorpedo(event) => event.cancel(model),
            TimelineEventType::Burn(event) => event.cancel(model),
        }
    }

    pub fn is_fire_torpedo(&self) -> bool {
        matches!(self, TimelineEventType::FireTorpedo(_))
    }

    pub fn is_burn(&self) -> bool {
        matches!(self, TimelineEventType::Burn(_))
    }

    pub fn as_fire_torpedo(&self) -> Option<FireTorpedoEvent> {
        if let TimelineEventType::FireTorpedo(fire_torpedo) = self {
            Some(fire_torpedo.clone())
        } else {
            None
        }
    }

    pub fn as_burn(&self) -> Option<BurnEvent> {
        if let TimelineEventType::Burn(burn) = self {
            Some(burn.clone())
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

    pub fn execute(&self, model: &mut Model) {
        debug!("Executing timeline event {:?}", self);
        self.type_.execute(model);
    }

    pub fn cancel(&self, model: &mut Model) {
        debug!("Cancelling timeline event {:?}", self);
        self.type_.cancel(model);
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
    pub fn events(&self) -> &VecDeque<TimelineEvent> {
        &self.events
    }

    /// # Panics
    /// Panics if the event occurs before the last event
    pub fn add(&mut self, event: TimelineEvent) {
        if let Some(last_event) = self.last_event() {
            assert!(event.time >= last_event.time);
        }
        self.events.push_back(event);
    }

    pub fn event_at_time(&self, time: f64) -> Option<&TimelineEvent> {
        self.events.iter().find(|event| event.time == time)
    }

    pub fn event_at_time_mut(&mut self, time: f64) -> Option<&mut TimelineEvent> {
        self.events.iter_mut().find(|event| event.time == time)
    }

    pub fn last_event(&self) -> Option<TimelineEvent> {
        self.events.back().cloned()
    }

    pub fn depleted_torpedoes(&self, slot_location :SlotLocation) -> usize {
        self.events.iter()
            .filter(|event| event.type_.as_fire_torpedo().is_some_and(|fire_torpedo| fire_torpedo.slot_location() == slot_location))
            .count()
    }

    /// Includes any event at `time`
    /// # Panics
    /// Panics if there is no last event
    pub fn pop_last_event(&mut self) -> TimelineEvent {
        self.events.pop_back().unwrap()
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn events_before(&mut self, time: f64) -> Vec<TimelineEvent> {
        let mut events = vec![];
        while self.events.front().is_some_and(|event| event.time < time) {
            events.push(self.events.front().unwrap().clone());
        }
        events
    }

    /// Does not include any event at `time`
    #[allow(clippy::missing_panics_doc)]
    pub fn pop_events_before(&mut self, time: f64) -> Vec<TimelineEvent> {
        let mut events = vec![];
        while self.events.front().is_some_and(|event| event.time < time) {
            events.push(self.events.pop_front().unwrap());
        }
        events
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}