use std::collections::VecDeque;

use intercept::InterceptEvent;
use log::trace;
use serde::{Deserialize, Serialize};
use start_turn::StartTurnEvent;

use crate::model::Model;

use self::{start_guidance::StartGuidanceEvent, fire_torpedo::FireTorpedoEvent, start_burn::StartBurnEvent};

pub mod intercept;
pub mod start_guidance;
pub mod start_burn;
pub mod start_turn;
pub mod fire_torpedo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TimelineEvent {
    Intercept(InterceptEvent),
    FireTorpedo(FireTorpedoEvent),
    StartBurn(StartBurnEvent),
    StartTurn(StartTurnEvent),
    StartGuidance(StartGuidanceEvent),
}

impl TimelineEvent {
    pub fn execute(&self, model: &mut Model) {
        match self {
            TimelineEvent::StartBurn(event) => event.execute(model),
            TimelineEvent::StartTurn(event) => event.execute(model),
            TimelineEvent::Intercept(event) => event.execute(model),
            TimelineEvent::StartGuidance(event) => event.execute(model),
            TimelineEvent::FireTorpedo(event) => event.execute(model),
        }
    }

    pub fn cancel(&self, model: &mut Model) {
        match self {
            TimelineEvent::StartBurn(event) => event.cancel(model),
            TimelineEvent::StartTurn(event) => event.cancel(model),
            TimelineEvent::Intercept(event) => event.cancel(model),
            TimelineEvent::StartGuidance(event) => event.cancel(model),
            TimelineEvent::FireTorpedo(event) => event.cancel(model),
        }
    }

    pub fn time(&self) -> f64 {
        match self {
            TimelineEvent::StartBurn(event) => event.time(),
            TimelineEvent::StartTurn(event) => event.time(),
            TimelineEvent::Intercept(event) => event.time(),
            TimelineEvent::StartGuidance(event) => event.time(),
            TimelineEvent::FireTorpedo(event) => event.time(),
        }
    }

    pub fn can_delete(&self, model: &Model) -> bool {
        match self {
            TimelineEvent::StartBurn(event) => event.can_remove(model),
            TimelineEvent::StartTurn(event) => event.can_remove(model),
            TimelineEvent::Intercept(event) => event.can_remove(),
            TimelineEvent::StartGuidance(event) => event.can_remove(model),
            TimelineEvent::FireTorpedo(event) => event.can_remove(),
        }
    }

    pub fn can_adjust(&self, model: &Model) -> bool {
        match self {
            TimelineEvent::StartBurn(event) => event.can_remove(model),
            TimelineEvent::StartTurn(event) => event.can_remove(model),
            TimelineEvent::Intercept(event) => event.can_remove(),
            TimelineEvent::StartGuidance(event) => event.can_remove(model),
            TimelineEvent::FireTorpedo(event) => event.can_remove(),
        }
    }

    /// Whether this event should prevent editing earlier events
    pub fn is_blocking(&self) -> bool {
        match self {
            TimelineEvent::StartBurn(event) => event.is_blocking(),
            TimelineEvent::StartTurn(event) => event.is_blocking(),
            TimelineEvent::Intercept(event) => event.is_blocking(),
            TimelineEvent::StartGuidance(event) => event.is_blocking(),
            TimelineEvent::FireTorpedo(event) => event.is_blocking(),
        }
    }

    pub fn is_start_burn(&self) -> bool {
        matches!(self, TimelineEvent::StartBurn(_))
    }

    pub fn is_start_turn(&self) -> bool {
        matches!(self, TimelineEvent::StartTurn(_))
    }

    pub fn is_intercept(&self) -> bool {
        matches!(self, TimelineEvent::Intercept(_))
    }
    
    pub fn is_enable_guidance(&self) -> bool {
        matches!(self, TimelineEvent::StartGuidance(_))
    }

    pub fn is_fire_torpedo(&self) -> bool {
        matches!(self, TimelineEvent::FireTorpedo(_))
    }

    pub fn as_start_burn(&self) -> Option<StartBurnEvent> {
        if let TimelineEvent::StartBurn(event_type) = self {
            Some(event_type.clone())
        } else {
            None
        }
    }

    pub fn as_start_turn(&self) -> Option<StartTurnEvent> {
        if let TimelineEvent::StartTurn(event_type) = self {
            Some(event_type.clone())
        } else {
            None
        }
    }

    pub fn as_intercept(&self) -> Option<InterceptEvent> {
        if let TimelineEvent::Intercept(event_type) = self {
            Some(event_type.clone())
        } else {
            None
        }
    }

    pub fn as_start_guidance(&self) -> Option<StartGuidanceEvent> {
        if let TimelineEvent::StartGuidance(event_type) = self {
            Some(event_type.clone())
        } else {
            None
        }
    }

    pub fn as_fire_torpedo(&self) -> Option<FireTorpedoEvent> {
        if let TimelineEvent::FireTorpedo(event_type) = self {
            Some(event_type.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
        let mut i = 0;
        while i < self.events.len() && self.events[i].time() < event.time() {
            i += 1;
        }
        trace!("Adding new timeline event {event:?} at index {i}");
        self.events.insert(i, event);
    }

    pub fn event_at_time(&self, time: f64) -> Option<&TimelineEvent> {
        self.events.iter().find(|event| event.time() == time)
    }

    pub fn last_event(&self) -> Option<TimelineEvent> {
        self.events.back().cloned()
    }

    pub fn last_fire_torpedo_event(&self) -> Option<FireTorpedoEvent> {
        self.events.iter()
            .rev()
            .find_map(TimelineEvent::as_fire_torpedo)
            .clone()
    }

    pub fn last_blocking_event(&self) -> Option<TimelineEvent> {
        self.events.iter()
            .rev()
            .find(|event| event.is_blocking())
            .cloned()
    }

    pub fn depleted_torpedoes(&self) -> usize {
        self.events.iter()
            .filter(|event| event.is_fire_torpedo())
            .count()
    }

    pub fn is_time_after_last_blocking_event(&self, time: f64) -> bool {
        match self.last_blocking_event() {
            Some(event) => event.time() <= time,
            None => true,
        }
    }

    /// Includes any event at `time`
    /// # Panics
    /// Panics if there is no last event
    pub fn pop_last_event(&mut self) -> TimelineEvent {
        self.events.pop_back().unwrap()
    }

    /// Does not include any event at `time`
    #[allow(clippy::missing_panics_doc)]
    pub fn pop_events_before(&mut self, time: f64) -> Vec<TimelineEvent> {
        let mut events = vec![];
        while self.events.front().is_some_and(|event| event.time() < time) {
            events.push(self.events.pop_front().unwrap());
        }
        events
    }
}
