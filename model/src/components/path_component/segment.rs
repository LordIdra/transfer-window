use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use super::{burn::Burn, orbit::Orbit};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Segment {
    Orbit(Orbit),
    Burn(Burn),
}

impl Segment {
    pub fn start_time(&self) -> f64 {
        match self {
            Segment::Orbit(orbit) => orbit.start_point().time(),
            Segment::Burn(burn) => burn.start_point().time(),
        }
    }

    pub fn start_position(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.start_point().position(),
            Segment::Burn(burn) => burn.start_point().position(),
        }
    }

    pub fn start_velocity(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.start_point().velocity(),
            Segment::Burn(burn) => burn.start_point().velocity(),
        }
    }

    pub fn current_position(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.current_point().position(),
            Segment::Burn(burn) => burn.current_point().position(),
        }
    }

    pub fn current_velocity(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.current_point().velocity(),
            Segment::Burn(burn) => burn.current_point().velocity(),
        }
    }

    pub fn end_time(&self) -> f64 {
        match self {
            Segment::Orbit(orbit) => orbit.end_point().time(),
            Segment::Burn(burn) => burn.end_point().time(),
        }
    }

    pub fn end_position(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.end_point().position(),
            Segment::Burn(burn) => burn.end_point().position(),
        }
    }

    pub fn end_velocity(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.end_point().velocity(),
            Segment::Burn(burn) => burn.end_point().velocity(),
        }
    }

    pub fn position_at_time(&self, time: f64) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.position_from_theta(orbit.theta_from_time(time)),
            Segment::Burn(burn) => burn.point_at_time(time).position(),
        }
    }

    pub fn velocity_at_time(&self, time: f64) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.velocity_from_theta(orbit.theta_from_time(time)),
            Segment::Burn(burn) => burn.point_at_time(time).velocity(),
        }
    }

    pub fn parent(&self) -> Entity {
        match self {
            Segment::Orbit(orbit) => orbit.parent(),
            Segment::Burn(burn) => burn.parent(),
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            Segment::Orbit(orbit) => orbit.is_finished(),
            Segment::Burn(burn) => burn.is_finished(),
        }
    }

    pub fn is_orbit(&self) -> bool {
        matches!(self, Segment::Orbit(_))
    }

    pub fn is_burn(&self) -> bool {
        matches!(self, Segment::Burn(_))
    }

    pub fn duration(&self) -> f64 {
        self.end_time() - self.start_time()
    }

    pub fn overshot_time(&self, time: f64) -> f64 {
        match self {
            Segment::Burn(burn) => burn.overshot_time(time),
            Segment::Orbit(orbit) => orbit.overshot_time(time),
        }
    }

    pub fn as_orbit(&self) -> Option<&Orbit> {
        match self {
            Segment::Burn(_) => None,
            Segment::Orbit(orbit) => Some(orbit),
        }
    }

    pub fn as_orbit_mut(&mut self) -> Option<&mut Orbit> {
        match self {
            Segment::Burn(_) => None,
            Segment::Orbit(orbit) => Some(orbit),
        }
    }

    pub fn as_burn(&self) -> Option<&Burn> {
        match self {
            Segment::Burn(burn) => Some(burn),
            Segment::Orbit(_) => None,
        }
    }

    pub fn as_burn_mut(&mut self) -> Option<&mut Burn> {
        match self {
            Segment::Burn(burn) => Some(burn),
            Segment::Orbit(_) => None,
        }
    }

    pub fn next(&mut self, delta_time: f64) {
        match self {
            Segment::Orbit(orbit) => orbit.next(delta_time),
            Segment::Burn(burn) => burn.next(delta_time),
        }
    }
}