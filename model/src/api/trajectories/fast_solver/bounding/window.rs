use std::cmp::Ordering;

use crate::{components::path_component::orbit::Orbit, storage::entity_allocator::Entity};

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

    pub fn orbit(&self) -> &'a Orbit {
        self.orbit
    }

    pub fn other_orbit(&self) -> &'a Orbit {
        self.other_orbit
    }

    pub fn other_entity(&self) -> Entity {
        self.other_entity
    }

    pub fn soonest_time(&self) -> f64 {
        f64::min(self.bound.0, self.bound.1)
    }

    pub fn latest_time(&self) -> f64 {
        f64::max(self.bound.0, self.bound.1)
    }

    pub fn is_periodic(&self) -> bool {
        self.periodic
    }

    // Increments bounds by period
    pub fn next(&self) -> Self {
        let bound = (self.bound.0 + self.orbit.period().unwrap(), self.bound.1 + self.orbit.period().unwrap());
        Self::new(self.orbit, self.other_orbit, self.other_entity, self.periodic, bound)
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        if self.soonest_time() > other.soonest_time() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}