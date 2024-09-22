use std::f64::consts::PI;

use nalgebra_glm::{vec2, DVec2};
use rust_kepler_solver::ellipse::EllipseSolver;
use serde::{Deserialize, Serialize};
use transfer_window_common::normalize_angle;

use crate::components::path_component::orbit::{orbit_direction::OrbitDirection, orbit_point::OrbitPoint, scary_math::{argument_of_periapsis, period, specific_angular_momentum}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ellipse {
    semi_major_axis: f64,
    eccentricity: f64,
    direction: OrbitDirection,
    period: f64,
    argument_of_periapsis: f64,
    specific_angular_momentum: f64,
    solver: EllipseSolver,
}

impl Ellipse {
    pub(in super) fn new(position: DVec2, velocity: DVec2, standard_gravitational_parameter: f64, semi_major_axis: f64, eccentricity: f64, direction: OrbitDirection) -> Self {
        let period = period(standard_gravitational_parameter, semi_major_axis);
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let specific_angular_momentum = specific_angular_momentum(position, velocity);
        let solver = EllipseSolver::new(eccentricity);
        Ellipse { semi_major_axis, eccentricity, direction, period, argument_of_periapsis, specific_angular_momentum, solver }
    }
}

impl Ellipse {
    pub fn theta_from_time_since_periapsis(&self, time_since_periapsis: f64) -> f64 {
        let time_since_periapsis = time_since_periapsis % self.period;
        let mean_anomaly = 2.0 * PI * time_since_periapsis / self.period;
        let eccentric_anomaly = self.solver.solve(mean_anomaly);
        let mut true_anomaly = 2.0 * f64::atan(f64::sqrt((1.0 + self.eccentricity) / (1.0 - self.eccentricity)) * f64::tan(eccentric_anomaly / 2.0));
        // The sign of atan flips halfway through the orbit
        // So we need to add 2pi halfway through the orbit to keep things consistent
        if let OrbitDirection::Clockwise = self.direction {
            true_anomaly = -true_anomaly;
        }
        let theta = true_anomaly + self.argument_of_periapsis;
        let theta = theta % (2.0 * PI);
        if theta < 0.0 {
            theta + 2.0 * PI
        } else {
            theta
        }
    }
    /// Always returns a positive time
    pub fn time_since_last_periapsis(&self, theta: f64) -> f64 {
        let true_anomaly = theta - self.argument_of_periapsis;
        let eccentric_anomaly = 2.0 * f64::atan(f64::sqrt((1.0 - self.eccentricity) / (1.0 + self.eccentricity)) * f64::tan(true_anomaly / 2.0));
        let mut mean_anomaly = eccentric_anomaly - self.eccentricity * f64::sin(eccentric_anomaly);
        if let OrbitDirection::Clockwise = self.direction {
            mean_anomaly = -mean_anomaly;
        }
        mean_anomaly = normalize_angle(mean_anomaly);
        mean_anomaly * self.period / (2.0 * PI)
    }

    pub fn position(&self, theta: f64) -> DVec2 {
        let true_anomaly = theta - self.argument_of_periapsis;
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * true_anomaly.cos());
        vec2(radius * theta.cos(), radius * theta.sin())
    }
    
    pub fn velocity(&self, position: DVec2, theta: f64) -> DVec2 {
        let true_anomaly = theta - self.argument_of_periapsis;
        let radius = position.magnitude();
        let radius_derivative_with_respect_to_theta = self.semi_major_axis * self.eccentricity * (1.0 - self.eccentricity.powi(2)) * true_anomaly.sin()
            / (self.eccentricity * true_anomaly.cos() + 1.0).powi(2);
        let position_derivative_with_respect_to_theta = vec2(
            radius_derivative_with_respect_to_theta * theta.cos() - radius * theta.sin(), 
            radius_derivative_with_respect_to_theta * theta.sin() + radius * theta.cos());
        let angular_speed = self.specific_angular_momentum / radius.powi(2);
        position_derivative_with_respect_to_theta * angular_speed
    }
    
    pub fn direction(&self) -> OrbitDirection {
        self.direction
    }

    pub fn period(&self) -> f64 {
        self.period
    }

    pub fn semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }

    pub fn semi_minor_axis(&self) -> f64 {
        self.semi_major_axis * f64::sqrt(1.0 - self.eccentricity.powi(2))
    }

    pub fn argument_of_periapsis(&self) -> f64 {
        self.argument_of_periapsis
    }

    pub fn eccentricity(&self) -> f64 {
        self.eccentricity
    }

    pub fn orbits(&self, time: f64) -> i32 {
        (time / self.period) as i32
    }

    pub fn is_time_between_points(start: &OrbitPoint, end: &OrbitPoint, time: f64) -> bool {
        time > start.time() && time < end.time()
    }
}

#[cfg(test)]
mod tests {
    use crate::components::path_component::{brute_force_tester::BruteForceTester, orbit::{conic::Conic, scary_math::{eccentricity, semi_major_axis, GRAVITATIONAL_CONSTANT}}};

    use super::*;
    
    #[test]
    fn test_time_since_last_periapsis_1() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = 1.53000e7;
        let eccentricity = 0.3725;
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(120.0);
        let time = ellipse.time_since_last_periapsis(theta);
        let expected_time = 1.13 * 60.0 * 60.0;
        assert!((time - expected_time).abs() < 30.0);
    }

    #[test]
    fn test_time_since_last_periapsis_2() {
        let position = vec2(147.095e9,  0.0);
        let velocity = vec2(0.0, 30.29e3);
        let parent_mass = 1.989e30;
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * parent_mass;
        let semi_major_axis = 149.598e9;
        let eccentricity = 0.016_710_22;
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let expected_time = 210.0 * 24.0 * 60.0 * 60.0;
        let duration = 365.20 * 24.0 * 60.0 * 60.0 + expected_time;
        let acceleration_from_time = |_: f64| vec2(0.0, 0.0);
        let mut tester = BruteForceTester::new(parent_mass, position, velocity, acceleration_from_time, 60.0);
        tester.update(duration);
        let theta = f64::atan2(tester.position().y, tester.position().x);
        let time = ellipse.time_since_last_periapsis(theta);
        assert!((expected_time - time).abs() < 1.0e4);
    }

    #[test]
    fn test_time_since_last_periapsis_3() {
        let position = vec2(8.0e6,  0.0);
        let velocity = vec2(0.0, 0.11e4);
        let parent_mass = 7.348e22;
        let conic = Conic::new(parent_mass, position, velocity);
        assert!(conic.semi_major_axis().is_sign_positive());
        let theta = 2.0;
        let time = conic.time_since_last_periapsis(theta);
        let new_theta = conic.theta_from_time_since_periapsis(time);
        assert!((new_theta - theta).abs() < 1.0e-4);
    }

    #[test]
    fn test_theta_from_time_since_periapsis_1() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = 1.53000e7;
        let eccentricity = 0.3725;
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let time = 3.0 * 3600.0;
        let theta = ellipse.theta_from_time_since_periapsis(time);
        let expected_theta = f64::to_radians(193.16 - 360.0) + 2.0*PI;
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_theta_from_time_since_periapsis_2() {
        let position = vec2(-83_760_632.160_125_73, -305_649_596.383_693_7);
        let velocity = vec2(-929.250_729_768_040_4, 1_168.034_466_965_014_9);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let expected_theta = f64::atan2(position.y, position.x) + 2.0*PI;
        let time = ellipse.time_since_last_periapsis(expected_theta);
        let theta = ellipse.theta_from_time_since_periapsis(time);
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_position_1() {
        let position = vec2(1.52100e11,  0.0);
        let velocity = vec2(0.0, 2.929e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.988_500e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = PI;
        let new_position = ellipse.position(true_anomaly);
        let expected_position = vec2(-1.470_703_941_8e11, 0.0);
        let position_difference = new_position - expected_position;
        assert!(position_difference.x.abs() < 5000.0);
        assert!(position_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_position_2() {
        let position = vec2(321_699_434.075_753_2, 238_177_462.813_335_57);
        let velocity = vec2(-448.885_375_943_825_5, 386.138_758_435_720_83);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = 0.637_311_079_175_916_3;
        let new_position = ellipse.position(theta);
        let position_difference = new_position - position;
        assert!(position_difference.x.abs() < 0.01);
        assert!(position_difference.y.abs() < 0.01);
    }

    #[test]
    fn test_velocity_1() {
        let position = vec2(1.52100e11,  0.0);
        let velocity = vec2(0.0, 2.929e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.988_500e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = PI;
        let new_position = ellipse.position(theta);
        let new_velocity = ellipse.velocity(new_position, theta);
        let expected_velocity = vec2(0.0, -3.029e4);
        let velocity_difference = new_velocity - expected_velocity;
        assert!(velocity_difference.x.abs() < 0.01);
        assert!(velocity_difference.y.abs() < 10.0);
    }

    #[test]
    fn test_velocity_2() {
        let position = vec2(234_851_481.381_961_97, 174_455_271.786_100_12);
        let velocity = vec2(-250.679_869_640_783_4, 817.559_112_681_255_2);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::atan2(position.y, position.x);
        let new_position = ellipse.position(theta);
        let new_velocity = ellipse.velocity(new_position, theta);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.01);
        assert!(velocity_difference.y.abs() < 0.01);
    }
}
