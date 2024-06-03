use std::f64::consts::PI;

use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::{storage::entity_allocator::Entity, util::normalize_angle};

use self::{conic::Conic, orbit_direction::OrbitDirection, orbit_point::OrbitPoint, scary_math::{sphere_of_influence, velocity_to_obtain_eccentricity, GRAVITATIONAL_CONSTANT}};

pub mod conic;
pub mod orbit_direction;
pub mod orbit_point;
pub mod scary_math;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Orbit {
    parent: Entity,
    mass: f64,
    conic: Conic,
    sphere_of_influence: f64,
    start_point: OrbitPoint,
    end_point: OrbitPoint,
    current_point: OrbitPoint,
}

impl Orbit {
    pub fn new(parent: Entity, mass: f64, parent_mass: f64, position: DVec2, velocity: DVec2, time: f64) -> Self {
        let conic = Conic::new(parent_mass, position, velocity);
        let sphere_of_influence = sphere_of_influence(mass, parent_mass, position, velocity);
        let start_point = OrbitPoint::new(&conic, position, time);
        let end_point = start_point.clone();
        let current_point = start_point.clone();
        Self { parent, mass, conic, sphere_of_influence, start_point, end_point, current_point }
    }

    pub fn circle(parent: Entity, mass: f64, parent_mass: f64, position: DVec2, time: f64, direction: OrbitDirection) -> Self {
        let conic = Conic::circle(parent_mass, position, direction);
        let standard_gravitational_parameter = parent_mass * GRAVITATIONAL_CONSTANT;
        let velocity = velocity_to_obtain_eccentricity(position, conic.eccentricity(), standard_gravitational_parameter, conic.semi_major_axis(), direction);
        let sphere_of_influence = sphere_of_influence(mass, parent_mass, position, velocity);
        let start_point = OrbitPoint::new(&conic, position, time);
        let end_point = start_point.clone();
        let current_point = start_point.clone();
        Self { parent, mass, conic, sphere_of_influence, start_point, end_point, current_point }
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn start_point(&self) -> &OrbitPoint {
        &self.start_point
    }

    pub fn current_point(&self) -> &OrbitPoint {
        &self.current_point
    }

    pub fn end_point(&self) -> &OrbitPoint {
        &self.end_point
    }

    pub fn point_at_time(&self, time: f64) -> OrbitPoint {
        let position = self.position_from_theta(self.theta_from_time(time));
        OrbitPoint::new(&self.conic, position, time)
    }

    pub fn remaining_angle(&self) -> f64 {
        // If we have any full orbits remaining, only return up to 2pi
        if self.remaining_orbits() > 0 {
            return 2.0 * PI;
        }

        let mut end_theta = normalize_angle(self.end_point.theta());
        let current_theta = normalize_angle(self.current_point.theta());
        if let OrbitDirection::AntiClockwise = self.conic.direction() {
            if end_theta < current_theta {
                end_theta += 2.0 * PI;
            }
        } else if end_theta > current_theta {
            end_theta -= 2.0 * PI;
        }
        end_theta - current_theta
    }

    pub fn remaining_orbits(&self) -> i32 {
        self.conic.orbits(self.end_point.time() - self.current_point.time())
    }

    pub fn completed_orbits(&self) -> i32 {
        self.conic.orbits(self.current_point.time() - self.start_point.time())
    }

    pub fn conic(&self) -> &Conic {
        &self.conic
    }

    pub fn parent(&self) -> Entity {
        self.parent
    }

    pub fn semi_major_axis(&self) -> f64 {
        self.conic.semi_major_axis()
    }

    pub fn semi_minor_axis(&self) -> f64 {
        self.conic.semi_minor_axis()
    }

    pub fn eccentricity(&self) -> f64 {
        self.conic.eccentricity()
    }

    pub fn argument_of_periapsis(&self) -> f64 {
        self.conic.argument_of_periapsis()
    }

    pub fn duration(&self) -> f64 {
        self.end_point.time() - self.start_point.time()
    }

    pub fn min_asymptote_theta(&self) -> Option<f64> {
        self.conic.min_asymptote_theta()
    }

    pub fn max_asymptote_theta(&self) -> Option<f64> {
        self.conic.max_asymptote_theta()
    }

    pub fn direction(&self) -> OrbitDirection {
        self.conic.direction()
    }

    pub fn period(&self) -> Option<f64> {
        self.conic.period()
    }

    pub fn is_clockwise(&self) -> bool {
        matches!(self.direction(), OrbitDirection::Clockwise)
    }

    pub fn is_ellipse(&self) -> bool {
        self.period().is_some()
    }

    pub fn is_finished(&self) -> bool {
        self.current_point.is_after(&self.end_point)
    }

    pub fn is_time_within_orbit(&self, time: f64) -> bool {
        time > self.start_point.time() && time < self.end_point.time()
    }

    pub fn time_since_first_periapsis(&self, theta: f64) -> f64 {
        let mut x = self.time_since_last_periapsis(theta);
        if let Some(period) = self.period() {
            x += period * self.completed_orbits() as f64;
        }
        x
    }

    pub fn time_since_last_periapsis(&self, theta: f64) -> f64 {
        self.conic.time_since_last_periapsis(theta)
    }

    pub fn first_periapsis_time(&self) -> f64 {
        self.start_point.time() - self.start_point.time_since_periapsis()
    }

    pub fn next_periapsis_time(&self) -> Option<f64> {
        let time = if let Some(period) = self.period() {
            let first_periapsis_time = self.first_periapsis_time();
            let time_since_first_periapsis = self.current_point.time() - first_periapsis_time;
            let orbits = (time_since_first_periapsis / period) as i32 + 1;
            self.first_periapsis_time() + orbits as f64 * period
        } else {
            self.first_periapsis_time()
        };

        if time > self.current_point.time() && time < self.end_point.time() {
            Some(time)
        } else {
            None
        }
    }

    pub fn next_apoapsis_time(&self) -> Option<f64> {
        if let Some(period) = self.period() {
            let mut first_apoapsis_time = self.first_periapsis_time() + period / 2.0;
            if first_apoapsis_time > self.start_point.time() {
                first_apoapsis_time -= period;
            }
            let time_since_first_apoapsis = self.current_point.time() - first_apoapsis_time;
            let orbits = (time_since_first_apoapsis / period) as i32 + 1;
            let time = first_apoapsis_time + orbits as f64 * period;
            if time > self.current_point.time() && time < self.end_point.time() {
                Some(time)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn theta_from_time(&self, time: f64) -> f64 {
        let time_since_periapsis = time - self.first_periapsis_time();
        self.conic.theta_from_time_since_periapsis(time_since_periapsis)
    }

    pub fn position_from_theta(&self, theta: f64) -> DVec2 {
        self.conic.position(theta)
    }

    pub fn velocity_from_theta(&self, theta: f64) -> DVec2 {
        self.conic.velocity(self.position_from_theta(theta), theta)
    }

    pub fn sphere_of_influence(&self) -> f64 {
        self.sphere_of_influence
    }

    pub fn end_at(&mut self, time: f64) {
        let theta = self.theta_from_time(time);
        let position = self.conic.position(theta);
        self.end_point = OrbitPoint::new(&self.conic, position, time);
    }

    pub fn with_end_at(mut self, time: f64) -> Self {
        self.end_at(time);
        self
    }

    pub fn reset(&mut self) {
        self.current_point = self.start_point.clone();
    }

    pub fn next(&mut self, time: f64) {
        let delta_time = time - self.current_point.time();
        self.current_point = self.current_point.next(&self.conic, delta_time);
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use nalgebra_glm::vec2;

    use crate::storage::entity_allocator::Entity;

    use super::Orbit;

    #[test]
    fn test_remaining_angle_1() {
        let parent = Entity::mock();
        let mass = 100.0;
        let parent_mass = 1.989e30;
        let position = vec2(147.095e9, 0.0);
        let velocity = vec2(0.0, 30.29e3);
        let start_time = 0.0;
        let end_time = 20.0 * 24.0 * 60.0 * 60.0;
        let mut orbit = Orbit::new(parent, mass, parent_mass, position, velocity, start_time);
        orbit.end_at(end_time);
        let expected_angle = orbit.theta_from_time(end_time);
        assert!((orbit.remaining_angle() - expected_angle).abs() < 1.0e-1);
    }

    #[test]
    fn test_remaining_angle_2() {
        let parent = Entity::mock();
        let mass = 100.0;
        let parent_mass = 1.989e30;
        let position = vec2(147.095e9, 0.0);
        let velocity = vec2(0.0, 30.29e3);
        let start_time = 0.0;
        let end_time = 283.0 * 24.0 * 60.0 * 60.0;
        let mut orbit = Orbit::new(parent, mass, parent_mass, position, velocity, start_time);
        orbit.end_at(end_time);
        let expected_angle = orbit.theta_from_time(end_time);
        assert!((orbit.remaining_angle() - expected_angle).abs() < 1.0e-1);
    }

    #[test]
    fn test_remaining_angle_3() {
        let parent = Entity::mock();
        let mass = 100.0;
        let parent_mass = 1.989e30;
        let position = vec2(147.095e9, 0.0);
        let velocity = vec2(0.0, 30.29e3);
        let start_time = 0.0;
        let end_time = 420.0 * 24.0 * 60.0 * 60.0;
        let mut orbit = Orbit::new(parent, mass, parent_mass, position, velocity, start_time);
        orbit.end_at(end_time);
        let expected_angle = 2.0 * PI;
        assert!((orbit.remaining_angle() - expected_angle).abs() < 1.0e-1);
    }
}