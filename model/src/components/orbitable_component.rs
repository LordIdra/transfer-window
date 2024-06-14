use nalgebra_glm::{vec2, DVec2};
use serde::{Deserialize, Serialize};

use super::path_component::{orbit::Orbit, segment::Segment};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum OrbitableType {
    Star,
    Planet,
    Moon,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrbitableComponentPhysics {
    Stationary(DVec2),
    Orbit(Segment), // stored as a segment because you can go from Segment to &Orbit but not vice versa
}

impl OrbitableComponentPhysics {
    pub fn position(&self) -> DVec2 {
        match self {
            OrbitableComponentPhysics::Stationary(position) => *position,
            OrbitableComponentPhysics::Orbit(orbit) => orbit.current_position(),
        }
    }

    pub fn position_at_time(&self, time: f64) -> DVec2 {
        match self {
            OrbitableComponentPhysics::Stationary(position) => *position,
            OrbitableComponentPhysics::Orbit(orbit) => orbit.position_at_time(time),
        }
    }

    pub fn velocity(&self) -> DVec2 {
        match self {
            OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
            OrbitableComponentPhysics::Orbit(orbit) => orbit.current_velocity(),
        }
    }

    pub fn velocity_at_time(&self, time: f64) -> DVec2 {
        match self {
            OrbitableComponentPhysics::Stationary(velocity) => *velocity,
            OrbitableComponentPhysics::Orbit(orbit) => orbit.velocity_at_time(time),
        }
    }
}

/// Must have `MassComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct OrbitableComponent {
    mass: f64,
    radius: f64,
    type_: OrbitableType,
    physics: OrbitableComponentPhysics,
}

impl OrbitableComponent {
    pub fn new(mass: f64, radius: f64, type_: OrbitableType, physics: OrbitableComponentPhysics) -> Self {
        Self { mass, radius, type_, physics }
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Returned segment is always an oribt
    pub fn segment(&self) -> Option<&Segment> {
        match &self.physics {
            OrbitableComponentPhysics::Stationary(_) => None,
            OrbitableComponentPhysics::Orbit(segment) => Some(segment),
        }
    }

    /// Returned segment is always an oribt
    pub fn orbit(&self) -> Option<&Orbit> {
        match &self.physics {
            OrbitableComponentPhysics::Stationary(_) => None,
            OrbitableComponentPhysics::Orbit(segment) => Some(segment.as_orbit().unwrap()),
        }
    }

    /// Returned segment is always an oribt
    pub fn orbit_mut(&mut self) -> Option<&mut Orbit> {
        match &mut self.physics {
            OrbitableComponentPhysics::Stationary(_) => None,
            OrbitableComponentPhysics::Orbit(segment) => Some(segment.as_orbit_mut().unwrap()),
        }
    }

    pub fn type_(&self) -> OrbitableType {
        self.type_
    }

    pub fn physics(&self) -> &OrbitableComponentPhysics {
        &self.physics
    }

    pub fn position(&self) -> DVec2 {
        self.physics.position()
    }

    pub fn position_at_time(&self, time: f64) -> DVec2 {
        self.physics.position_at_time(time)
    }

    pub fn velocity(&self) -> DVec2 {
        self.physics.velocity()
    }

    pub fn velocity_at_time(&self, time: f64) -> DVec2 {
        self.physics.velocity_at_time(time)
    }
}