use std::cmp::Ordering;

use crate::{components::trajectory_component::orbit::Orbit, constants::SOLVER_WINDOW_MAX_SECONDS, storage::entity_allocator::Entity};

/// A one-time window where the bounds can be checked once then discarded
#[derive(Debug)]
pub struct Window<'a> {
    orbit: &'a Orbit,
    other_orbit: &'a Orbit,
    other_entity: Entity,
    periodic: bool,
    bound: (f64, f64),
}

impl<'a> Window<'a> {
    pub fn new(orbit: &'a Orbit, other_orbit: &'a Orbit, other_entity: Entity, periodic: bool, bound: (f64, f64)) -> Self {
        Self { orbit, other_orbit, other_entity, periodic, bound }
    }

    pub fn get_orbit(&self) -> &'a Orbit {
        self.orbit
    }

    pub fn get_other_orbit(&self) -> &'a Orbit {
        self.other_orbit
    }

    pub fn get_other_entity(&self) -> Entity {
        self.other_entity
    }

    pub fn get_soonest_time(&self) -> f64 {
        f64::min(self.bound.0, self.bound.1)
    }

    pub fn get_latest_time(&self) -> f64 {
        f64::max(self.bound.0, self.bound.1)
    }

    pub fn is_periodic(&self) -> bool {
        self.periodic
    }

    // Increments bounds by period
    pub fn next(&self) -> Self {
        let bound = (self.bound.0 + self.orbit.get_period().unwrap(), self.bound.1 + self.orbit.get_period().unwrap());
        Self::new(self.orbit, self.other_orbit, self.other_entity, self.periodic, bound)
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        if self.get_soonest_time() > other.get_soonest_time() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    pub fn split(self) -> Vec<Window<'a>> {
        let mut windows = Vec::new();
        let mut from = self.get_soonest_time();
        let mut to = f64::min(self.get_latest_time(), from + SOLVER_WINDOW_MAX_SECONDS);
        while from != to {
            windows.push(Window::new(self.orbit, self.other_orbit, self.other_entity, self.periodic, (from, to)));
            from = to;
            to = f64::min(self.get_latest_time(), from + SOLVER_WINDOW_MAX_SECONDS);
        }
        return windows;
    }
}