use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use super::conic::Conic;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrbitPoint {
    theta: f64,
    time: f64,
    time_since_periapsis: f64,
    position: DVec2,
    velocity: DVec2,
}

impl OrbitPoint {
    pub fn new(conic: &Conic, position: DVec2, time: f64) -> Self {
        let theta = f64::atan2(position.y, position.x);
        let time_since_periapsis = conic.time_since_last_periapsis(theta);
        let velocity = conic.velocity(position, theta);
        Self {
            theta,
            time,
            time_since_periapsis,
            position,
            velocity,
        }
    }

    pub fn next(&self, conic: &Conic, delta_time: f64) -> Self {
        let time = self.time + delta_time;
        let mut time_since_periapsis = self.time_since_periapsis + delta_time;
        if let Some(period) = conic.period() {
            if time_since_periapsis > period {
                time_since_periapsis -= period;
            }
        }
        let theta = conic.theta_from_time_since_periapsis(time_since_periapsis);
        let position = conic.position(theta);
        let velocity = conic.velocity(position, theta);
        Self {
            theta,
            time,
            time_since_periapsis,
            position,
            velocity,
        }
    }

    pub fn theta(&self) -> f64 {
        self.theta
    }

    pub fn position(&self) -> DVec2 {
        self.position
    }

    pub fn velocity(&self) -> DVec2 {
        self.velocity
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn time_since_periapsis(&self) -> f64 {
        self.time_since_periapsis
    }

    pub fn is_after(&self, other: &OrbitPoint) -> bool {
        self.time > other.time
    }
}
