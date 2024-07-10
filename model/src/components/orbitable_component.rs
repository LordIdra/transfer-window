use nalgebra_glm::{DVec2, vec2};
use serde::{Deserialize, Serialize};

use super::path_component::{orbit::Orbit, segment::Segment};

use self::atmosphere::Atmosphere;

pub mod atmosphere;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum OrbitableType {
    Star,
    Planet,
    Moon,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrbitableComponent {
    mass: f64,
    radius: f64,
    rotation_period: f64,
    rotation_angle: f64,
    type_: OrbitableType,
    physics: OrbitableComponentPhysics,
    atmosphere: Option<Atmosphere>
}

impl OrbitableComponent {
    pub fn new(
        mass: f64,
        radius: f64,
        rotation_period_in_days: f64,
        rotation_angle: f64,
        type_: OrbitableType,
        physics: OrbitableComponentPhysics,
        atmosphere: Option<Atmosphere>,
    ) -> Self {
        let rotation_period = rotation_period_in_days * 24.0 * 60.0 * 60.0;
        Self { mass, radius, rotation_period, rotation_angle, type_, physics, atmosphere }
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Returned segment is always an orbit
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
    
    pub fn rotation_period_in_secs(&self) -> f64 {
        self.rotation_period
    }
    
    pub fn rotation_angle(&self) -> f64 {
        self.rotation_angle
    }
    
    pub fn set_rotation_angle(&mut self, rotation_angle: f64) {
        self.rotation_angle = rotation_angle;
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
    
    pub fn atmosphere(&self) -> Option<&Atmosphere> {
        self.atmosphere.as_ref()
    }
}