use std::f64::consts::PI;

use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::{storage::entity_allocator::Entity, util::normalize_angle};

use self::{conic::{Conic, ConicType}, orbit_direction::OrbitDirection, orbit_point::OrbitPoint, scary_math::{sphere_of_influence, velocity_to_obtain_eccentricity, GRAVITATIONAL_CONSTANT}};

pub mod conic;
pub mod orbit_direction;
pub mod orbit_point;
pub mod scary_math;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Orbit {
    parent: Entity,
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
        Self { parent, conic, sphere_of_influence, start_point, end_point, current_point }
    }

    pub fn circle(parent: Entity, mass: f64, parent_mass: f64, position: DVec2, time: f64, direction: OrbitDirection) -> Self {
        let conic = Conic::circle(parent_mass, position, direction);
        let standard_gravitational_parameter = parent_mass * GRAVITATIONAL_CONSTANT;
        let velocity = velocity_to_obtain_eccentricity(position, conic.get_eccentricity(), standard_gravitational_parameter, conic.get_semi_major_axis(), direction);
        let sphere_of_influence = sphere_of_influence(mass, parent_mass, position, velocity);
        let start_point = OrbitPoint::new(&conic, position, time);
        let end_point = start_point.clone();
        let current_point = start_point.clone();
        Self { parent, conic, sphere_of_influence, start_point, end_point, current_point }
    }

    pub fn get_start_point(&self) -> &OrbitPoint {
        &self.start_point
    }

    pub fn get_current_point(&self) -> &OrbitPoint {
        &self.current_point
    }

    pub fn get_end_point(&self) -> &OrbitPoint {
        &self.end_point
    }

    pub fn get_remaining_angle(&self) -> f64 {
        // If we have any full orbits remaining, only return up to 2pi
        if self.get_remaining_orbits() > 0 {
            return 2.0 * PI;
        }

        let mut end_theta = normalize_angle(self.end_point.get_theta());
        let current_theta = normalize_angle(self.current_point.get_theta());
        if let OrbitDirection::AntiClockwise = self.conic.get_direction() {
            if end_theta < current_theta {
                end_theta += 2.0 * PI;
            }
        } else if end_theta > current_theta {
            end_theta -= 2.0 * PI;
        }
        end_theta - current_theta
    }

    pub fn get_remaining_orbits(&self) -> i32 {
        self.conic.get_orbits(self.end_point.get_time() - self.current_point.get_time())
    }

    pub fn get_completed_orbits(&self) -> i32 {
        self.conic.get_orbits(self.current_point.get_time() - self.start_point.get_time())
    }

    pub fn get_conic_type(&self) -> ConicType {
        self.conic.get_type()
    }

    pub fn get_parent(&self) -> Entity {
        self.parent
    }

    pub fn get_semi_major_axis(&self) -> f64 {
        self.conic.get_semi_major_axis()
    }

    pub fn get_semi_minor_axis(&self) -> f64 {
        self.conic.get_semi_minor_axis()
    }

    pub fn get_eccentricity(&self) -> f64 {
        self.conic.get_eccentricity()
    }

    pub fn get_argument_of_periapsis(&self) -> f64 {
        self.conic.get_argument_of_periapsis()
    }

    pub fn get_duration(&self) -> f64 {
        self.end_point.get_time() - self.start_point.get_time()
    }

    pub fn get_min_asymptote_theta(&self) -> Option<f64> {
        self.conic.get_min_asymptote_theta()
    }

    pub fn get_max_asymptote_theta(&self) -> Option<f64> {
        self.conic.get_max_asymptote_theta()
    }

    pub fn get_direction(&self) -> OrbitDirection {
        self.conic.get_direction()
    }

    pub fn get_period(&self) -> Option<f64> {
        self.conic.get_period()
    }

    pub fn is_clockwise(&self) -> bool {
        match self.get_direction() {
            OrbitDirection::Clockwise => true,
            OrbitDirection::AntiClockwise => false,
        }
    }

    pub fn is_ellipse(&self) -> bool {
        self.get_period().is_some()
    }

    pub fn is_finished(&self) -> bool {
        self.current_point.is_after(&self.end_point)
    }

    pub fn is_time_within_orbit(&self, time: f64) -> bool {
        time > self.start_point.get_time() && time < self.end_point.get_time()
    }

    pub fn get_overshot_time(&self, time: f64) -> f64 {
        time - self.end_point.get_time()
    }

    pub fn get_time_since_first_periapsis(&self, theta: f64) -> f64 {
        let mut x = self.get_time_since_last_periapsis(theta);
        if let Some(period) = self.get_period() {
            x += period * self.get_completed_orbits() as f64;
        }
        x
    }

    pub fn get_time_since_last_periapsis(&self, theta: f64) -> f64 {
        self.conic.get_time_since_last_periapsis(theta)
    }

    pub fn get_first_periapsis_time(&self) -> f64 {
        self.start_point.get_time() - self.start_point.get_time_since_periapsis()
    }

    pub fn get_theta_from_time(&self, time: f64) -> f64 {
        let time_since_periapsis = time - self.get_first_periapsis_time();
        self.conic.get_theta_from_time_since_periapsis(time_since_periapsis)
    }

    pub fn get_position_from_theta(&self, theta: f64) -> DVec2 {
        self.conic.get_position(theta)
    }

    pub fn get_velocity_from_theta(&self, theta: f64) -> DVec2 {
        self.conic.get_velocity(self.get_position_from_theta(theta), theta)
    }

    pub fn get_sphere_of_influence(&self) -> f64 {
        self.sphere_of_influence
    }

    pub fn end_at(&mut self, time: f64) {
        let theta = self.get_theta_from_time(time);
        let position = self.conic.get_position(theta);
        self.end_point = OrbitPoint::new(&self.conic, position, time);
    }

    pub fn reset(&mut self) {
        self.current_point = self.start_point.clone();
    }

    pub fn next(&mut self, delta_time: f64) {
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
    fn test_get_remaining_angle_1() {
        let parent = Entity::mock();
        let mass = 100.0;
        let parent_mass = 1.989e30;
        let position = vec2(147.095e9, 0.0);
        let velocity = vec2(0.0, 30.29e3);
        let start_time = 0.0;
        let end_time = 20.0 * 24.0 * 60.0 * 60.0;
        let mut orbit = Orbit::new(parent, mass, parent_mass, position, velocity, start_time);
        orbit.end_at(end_time);
        let expected_angle = orbit.get_theta_from_time(end_time);
        assert!((orbit.get_remaining_angle() - expected_angle).abs() < 1.0e-1);
    }

    #[test]
    fn test_get_remaining_angle_2() {
        let parent = Entity::mock();
        let mass = 100.0;
        let parent_mass = 1.989e30;
        let position = vec2(147.095e9, 0.0);
        let velocity = vec2(0.0, 30.29e3);
        let start_time = 0.0;
        let end_time = 283.0 * 24.0 * 60.0 * 60.0;
        let mut orbit = Orbit::new(parent, mass, parent_mass, position, velocity, start_time);
        orbit.end_at(end_time);
        let expected_angle = orbit.get_theta_from_time(end_time);
        assert!((orbit.get_remaining_angle() - expected_angle).abs() < 1.0e-1);
    }

    #[test]
    fn test_get_remaining_angle_3() {
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
        assert!((orbit.get_remaining_angle() - expected_angle).abs() < 1.0e-1);
    }
}