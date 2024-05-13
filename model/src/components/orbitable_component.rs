use std::time::Duration;
use nalgebra_glm::{vec2, DVec2};
use serde::{Deserialize, Serialize};

use super::path_component::orbit::Orbit;

#[derive(Debug, Serialize, Deserialize)]
pub enum OrbitableComponentPhysics {
    Stationary(DVec2),
    Orbit(Orbit),
}

impl OrbitableComponentPhysics {
    pub fn position(&self) -> DVec2 {
        match self {
            OrbitableComponentPhysics::Stationary(position) => *position,
            OrbitableComponentPhysics::Orbit(orbit) => orbit.current_point().position(),
        }
    }

    pub fn position_at_time(&self, time: f64) -> DVec2 {
        match self {
            OrbitableComponentPhysics::Stationary(position) => *position,
            OrbitableComponentPhysics::Orbit(orbit) => orbit.position_from_theta(orbit.theta_from_time(time)),
        }
    }

    pub fn velocity(&self) -> DVec2 {
        match self {
            OrbitableComponentPhysics::Stationary(_) => vec2(0.0, 0.0),
            OrbitableComponentPhysics::Orbit(orbit) => orbit.current_point().velocity(),
        }
    }

    pub fn velocity_at_time(&self, time: f64) -> DVec2 {
        match self {
            OrbitableComponentPhysics::Stationary(velocity) => *velocity,
            OrbitableComponentPhysics::Orbit(orbit) => orbit.velocity_from_theta(orbit.theta_from_time(time)),
        }
    }
}

/// Must have `MassComponent`
#[derive(Debug, Serialize, Deserialize)]
pub struct OrbitableComponent {
    mass: f64,
    radius: f64,
    rotation_period: Duration,
    cumulative_rotation: Duration,
    physics: OrbitableComponentPhysics,
}

impl OrbitableComponent {
    pub fn new(mass: f64, radius: f64, rotation_period: Duration, physics: OrbitableComponentPhysics) -> Self {
        let cumulative_rotation = Duration::from_secs(0);
        Self { mass, radius, rotation_period, cumulative_rotation, physics }
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
    
    pub fn rotation_period(&self) -> Duration {
        self.rotation_period
    }
    
    pub fn rotation(&self) -> f32 {
        let rotation_period = self.rotation_period.as_secs_f32();
        let cumulative_rotation = self.cumulative_rotation.as_secs_f32();
        cumulative_rotation / rotation_period * 2.0 * std::f32::consts::PI
    }
    
    pub fn add_rotation(&mut self, duration: Duration) {
        self.cumulative_rotation += duration;
    }

    pub fn orbit(&self) -> Option<&Orbit> {
        match &self.physics {
            OrbitableComponentPhysics::Stationary(_) => None,
            OrbitableComponentPhysics::Orbit(orbit) => Some(orbit),
        }
    }

    pub fn orbit_mut(&mut self) -> Option<&mut Orbit> {
        match &mut self.physics {
            OrbitableComponentPhysics::Stationary(_) => None,
            OrbitableComponentPhysics::Orbit(orbit) => Some(orbit),
        }
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