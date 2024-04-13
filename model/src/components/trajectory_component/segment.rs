use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use super::{burn::Burn, orbit::Orbit};

#[derive(Debug, Serialize, Deserialize)]
pub enum Segment {
    Orbit(Orbit),
    Burn(Burn),
}

impl Segment {
    pub fn get_start_time(&self) -> f64 {
        match self {
            Segment::Orbit(orbit) => orbit.get_start_point().get_time(),
            Segment::Burn(burn) => burn.get_start_point().get_time(),
        }
    }

    pub fn get_current_position(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.get_current_point().get_position(),
            Segment::Burn(burn) => burn.get_current_point().get_position(),
        }
    }

    pub fn get_current_velocity(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.get_current_point().get_velocity(),
            Segment::Burn(burn) => burn.get_current_point().get_velocity(),
        }
    }

    pub fn get_end_time(&self) -> f64 {
        match self {
            Segment::Orbit(orbit) => orbit.get_end_point().get_time(),
            Segment::Burn(burn) => burn.get_end_point().get_time(),
        }
    }

    pub fn get_end_position(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.get_end_point().get_position(),
            Segment::Burn(burn) => burn.get_end_point().get_position(),
        }
    }

    pub fn get_end_velocity(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.get_end_point().get_velocity(),
            Segment::Burn(burn) => burn.get_end_point().get_velocity(),
        }
    }

    pub fn get_position_at_time(&self, time: f64) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.get_position_from_theta(orbit.get_theta_from_time(time)),
            Segment::Burn(burn) => burn.get_point_at_time(time).get_position(),
        }
    }

    pub fn get_velocity_at_time(&self, time: f64) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.get_velocity_from_theta(orbit.get_theta_from_time(time)),
            Segment::Burn(burn) => burn.get_point_at_time(time).get_velocity(),
        }
    }

    pub fn get_parent(&self) -> Entity {
        match self {
            Segment::Orbit(orbit) => orbit.get_parent(),
            Segment::Burn(burn) => burn.get_parent(),
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            Segment::Orbit(orbit) => orbit.is_finished(),
            Segment::Burn(burn) => burn.is_finished(),
        }
    }

    pub fn get_overshot_time(&self, time: f64) -> f64 {
        match self {
            Segment::Burn(burn) => burn.get_overshot_time(time),
            Segment::Orbit(orbit) => orbit.get_overshot_time(time),
        }
    }

    /// # Panics
    /// Panics if the segment is not an orbit
    pub fn as_orbit(&self) -> &Orbit {
        match self {
            Segment::Burn(_) => panic!("Attempted to get non-orbit segment as orbit"),
            Segment::Orbit(orbit) => orbit,
        }
    }

    /// # Panics
    /// Panics if the segment is not a burn
    pub fn as_orbit_mut(&mut self) -> &mut Orbit {
        match self {
            Segment::Burn(_) => panic!("Attempted to get non-orbit segment as orbit"),
            Segment::Orbit(orbit) => orbit,
        }
    }

    /// # Panics
    /// Panics if the segment is not an orbit
    pub fn as_burn(&self) -> &Burn {
        match self {
            Segment::Burn(burn) => burn,
            Segment::Orbit(_) => panic!("Attempted to get non-burn segment as burn"),
        }
    }

    pub fn next(&mut self, delta_time: f64) {
        match self {
            Segment::Orbit(orbit) => orbit.next(delta_time),
            Segment::Burn(burn) => burn.next(delta_time),
        }
    }
}