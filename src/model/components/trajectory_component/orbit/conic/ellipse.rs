use std::f64::consts::PI;

use nalgebra_glm::{vec2, DVec2};

use crate::{model::components::trajectory_component::orbit::{conic_type::ConicType, orbit_direction::OrbitDirection, orbit_point::OrbitPoint, scary_math::{argument_of_periapsis, kepler_ellipse::EllipseSolver, period, specific_angular_momentum}}, util::normalize_angle};

use super::Conic;

#[derive(Debug)]
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
        Ellipse { semi_major_axis, eccentricity, period, argument_of_periapsis, direction, specific_angular_momentum, solver }
    }
}

impl Conic for Ellipse {
    fn get_theta_from_time_since_periapsis(&self, time_since_periapsis: f64) -> f64 {
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
    fn get_time_since_last_periapsis(&self, theta: f64) -> f64 {
        let mut true_anomaly = theta - self.argument_of_periapsis;
        // Solve an edge case where if true_anomaly is very close to 0 or pi, it will spit out inaccurate results due to the tan
        if true_anomaly.abs() < 1.0e-6 || (true_anomaly.abs() - PI).abs() < 1.0e-4 {
            true_anomaly += 1.0e-4
        }
        let eccentric_anomaly = 2.0 * f64::atan(f64::sqrt((1.0 - self.eccentricity) / (1.0 + self.eccentricity)) * f64::tan(true_anomaly / 2.0));
        let mut mean_anomaly = eccentric_anomaly - self.eccentricity * f64::sin(eccentric_anomaly);
        if let OrbitDirection::Clockwise = self.direction {
            mean_anomaly = -mean_anomaly;
        }
        mean_anomaly = normalize_angle(mean_anomaly);
        mean_anomaly * self.period / (2.0 * PI)
    }

    fn get_position(&self, theta: f64) -> DVec2 {
        let true_anomaly = theta - self.argument_of_periapsis;
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * true_anomaly.cos());
        vec2(radius * theta.cos(), radius * theta.sin())
    }
    
    fn get_velocity(&self, position: DVec2, theta: f64) -> DVec2 {
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

    fn get_type(&self) -> ConicType {
        ConicType::Ellipse
    }

    fn get_direction(&self) -> OrbitDirection {
        self.direction
    }

    fn get_period(&self) -> Option<f64> {
        Some(self.period)
    }

    fn get_semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }

    fn get_semi_minor_axis(&self) -> f64 {
        self.semi_major_axis * f64::sqrt(1.0 - self.eccentricity.powi(2))
    }

    fn get_argument_of_periapsis(&self) -> f64 {
        self.argument_of_periapsis
    }

    fn get_min_asymptote_theta(&self) -> Option<f64> {
        None
    }

    fn get_max_asymptote_theta(&self) -> Option<f64> {
        None
    }

    fn get_eccentricity(&self) -> f64 {
        self.eccentricity
    }

    fn get_orbits(&self, time: f64) -> i32 {
        (time / self.period) as i32
    }

    fn is_time_between_points(&self, start: &OrbitPoint, end: &OrbitPoint, time: f64) -> bool {
        time > start.get_time() && time < end.get_time()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::components::trajectory_component::{brute_force_tester::BruteForceTester, orbit::{conic::new_conic, scary_math::{eccentricity, semi_major_axis, GRAVITATIONAL_CONSTANT}}};

    use super::*;
    
    #[test]
    fn test_get_time_since_last_periapsis_1() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = 1.53000e7;
        let eccentricity = 0.3725;
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(120.0);
        let time = ellipse.get_time_since_last_periapsis(theta);
        let expected_time = 1.13 * 60.0 * 60.0;
        assert!((time - expected_time).abs() < 30.0);
    }

    #[test]
    fn test_get_time_since_last_periapsis_2() {
        let position = vec2(147.095e9,  0.0);
        let velocity = vec2(0.0, 30.29e3);
        let parent_mass = 1.989e30;
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * parent_mass;
        let semi_major_axis = 149.598e9;
        let eccentricity = 0.01671022;
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let expected_time = 210.0 * 24.0 * 60.0 * 60.0;
        let duration = 365.20 * 24.0 * 60.0 * 60.0 + expected_time;
        let mut tester = BruteForceTester::new(parent_mass, position, velocity, vec2(0.0, 0.0), 60.0);
        tester.update(duration);
        let theta = f64::atan2(tester.get_position().y, tester.get_position().x);
        let time = ellipse.get_time_since_last_periapsis(theta);
        assert!((expected_time - time).abs() < 1.0e4);
    }

    #[test]
    fn test_get_time_since_last_periapsis_3() {
        let position = vec2(8.0e6,  0.0);
        let velocity = vec2(0.0, 0.11e4);
        let parent_mass = 7.348e22;
        let conic = new_conic(parent_mass, position, velocity);
        assert!(conic.get_semi_major_axis().is_sign_positive());
        let theta = 2.0;
        let time = conic.get_time_since_last_periapsis(theta);
        let new_theta = conic.get_theta_from_time_since_periapsis(time);
        assert!((new_theta - theta).abs() < 1.0e-4);
    }

    #[test]
    fn test_get_theta_from_time_since_periapsis_1() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = 1.53000e7;
        let eccentricity = 0.3725;
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let time = 3.0 * 3600.0;
        let theta = ellipse.get_theta_from_time_since_periapsis(time);
        let expected_theta = f64::to_radians(193.16 - 360.0) + 2.0*PI;
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_get_theta_from_time_since_periapsis_2() {
        let position = vec2(-83760632.16012573, -305649596.3836937);
        let velocity = vec2(-929.2507297680404, 1168.0344669650149);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let expected_theta = f64::atan2(position.y, position.x) + 2.0*PI;
        let time = ellipse.get_time_since_last_periapsis(expected_theta);
        let theta = ellipse.get_theta_from_time_since_periapsis(time);
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_get_position_1() {
        let position = vec2(1.52100e11,  0.0);
        let velocity = vec2(0.0, 2.929e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.988500e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = PI;
        let new_position = ellipse.get_position(true_anomaly);
        let expected_position = vec2(-1.4707039418e11, 0.0);
        let position_difference = new_position - expected_position;
        assert!(position_difference.x.abs() < 5000.0);
        assert!(position_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_get_position_2() {
        let position = vec2(321699434.0757532, 238177462.81333557);
        let velocity = vec2(-448.8853759438255, 386.13875843572083);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = 0.6373110791759163;
        let new_position = ellipse.get_position(theta);
        let position_difference = new_position - position;
        assert!(position_difference.x.abs() < 0.01);
        assert!(position_difference.y.abs() < 0.01);
    }

    #[test]
    fn test_get_velocity_1() {
        let position = vec2(1.52100e11,  0.0);
        let velocity = vec2(0.0, 2.929e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.988500e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = PI;
        let new_position = ellipse.get_position(theta);
        let new_velocity = ellipse.get_velocity(new_position, theta);
        let expected_velocity = vec2(0.0, -3.029e4);
        let velocity_difference = new_velocity - expected_velocity;
        assert!(velocity_difference.x.abs() < 0.01);
        assert!(velocity_difference.y.abs() < 10.0);
    }

    #[test]
    fn test_get_velocity_2() {
        let position = vec2(234851481.38196197, 174455271.78610012);
        let velocity = vec2(-250.6798696407834, 817.5591126812552);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::atan2(position.y, position.x);
        let new_position = ellipse.get_position(theta);
        let new_velocity = ellipse.get_velocity(new_position, theta);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.01);
        assert!(velocity_difference.y.abs() < 0.01);
    }
}