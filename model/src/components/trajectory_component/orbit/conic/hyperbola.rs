use std::f64::consts::PI;

use nalgebra_glm::{vec2, DVec2};
use serde::{Deserialize, Serialize};

use crate::components::trajectory_component::orbit::{orbit_direction::OrbitDirection, orbit_point::OrbitPoint, scary_math::{argument_of_periapsis, asymptote_theta, kepler_hyperbola::HyperbolaSolver, specific_angular_momentum}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hyperbola {
    standard_gravitational_parameter: f64,
    semi_major_axis: f64,
    eccentricity: f64,
    direction: OrbitDirection,
    argument_of_periapsis: f64,
    min_asymptote_theta: f64,
    max_asymptote_theta: f64,
    specific_angular_momentum: f64,
    solver: HyperbolaSolver,
}

impl Hyperbola {
    pub(in super) fn new(position: DVec2, velocity: DVec2, standard_gravitational_parameter: f64, semi_major_axis: f64, eccentricity: f64, direction: OrbitDirection) -> Self {
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let (min_asymptote_theta, max_asymptote_theta) = asymptote_theta(eccentricity, argument_of_periapsis);
        let specific_angular_momentum = specific_angular_momentum(position, velocity);
        let solver = HyperbolaSolver::new(eccentricity);
        Hyperbola { standard_gravitational_parameter, semi_major_axis, eccentricity, direction, argument_of_periapsis, min_asymptote_theta, max_asymptote_theta, specific_angular_momentum, solver }
    }
}

impl Hyperbola {
    pub fn get_theta_from_time_since_periapsis(&self, time_since_periapsis: f64) -> f64 {
        let x = self.standard_gravitational_parameter.powi(2) / self.specific_angular_momentum.powi(3);
        let mean_anomaly = x * time_since_periapsis * (self.eccentricity.powi(2) - 1.0).powf(3.0 / 2.0);
        let eccentric_anomaly = self.solver.solve(mean_anomaly);
        let true_anomaly = 2.0 * f64::atan(f64::sqrt((self.eccentricity + 1.0) / (self.eccentricity - 1.0)) * f64::tanh(eccentric_anomaly / 2.0));
        let theta = true_anomaly + self.argument_of_periapsis;
        let theta = theta % (2.0 * PI);
        if theta < 0.0 {
            theta + 2.0 * PI
        } else {
            theta
        }
    }

    /// Time can be negative if we have not reached the periapsis at the given theta
    pub fn get_time_since_last_periapsis(&self, theta: f64) -> f64 {
        let mut true_anomaly = theta - self.argument_of_periapsis;
        // Solve an edge case where if true_anomaly is very close to 0 or pi, it will spit out inaccurate results due to the tan
        if true_anomaly.abs() < 1.0e-6 || (true_anomaly.abs() - PI).abs() < 1.0e-4 {
            true_anomaly += 1.0e-4;
        }
        let eccentric_anomaly = 2.0 * f64::atanh(f64::sqrt((self.eccentricity - 1.0) / (self.eccentricity + 1.0)) * f64::tan(true_anomaly / 2.0));
        let mean_anomaly = self.eccentricity * f64::sinh(eccentric_anomaly) - eccentric_anomaly;
        let x = self.specific_angular_momentum.powi(3) / self.standard_gravitational_parameter.powi(2);
        mean_anomaly * x / (self.eccentricity.powi(2) - 1.0).powf(3.0 / 2.0)
    }

    pub fn get_position(&self, theta: f64) -> DVec2 {
        let true_anomaly = theta - self.argument_of_periapsis;
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * true_anomaly.cos());
        vec2(radius * theta.cos(), radius * theta.sin())
    }
    
    pub fn get_velocity(&self, position: DVec2, theta: f64) -> DVec2 {
        let true_anomaly = theta - self.argument_of_periapsis;
        let radius = position.magnitude();
        let radius_derivative_with_respect_to_true_anomaly = self.semi_major_axis * self.eccentricity * (1.0 - self.eccentricity.powi(2)) * true_anomaly.sin()
            / (self.eccentricity * true_anomaly.cos() + 1.0).powi(2);
        let position_derivative_with_respect_to_true_anomaly = vec2(
            radius_derivative_with_respect_to_true_anomaly * theta.cos() - radius * theta.sin(), 
            radius_derivative_with_respect_to_true_anomaly * theta.sin() + radius * theta.cos());
        let angular_speed = self.specific_angular_momentum / radius.powi(2);
        position_derivative_with_respect_to_true_anomaly * angular_speed
    }

    pub fn get_direction(&self) -> OrbitDirection {
        self.direction
    }

    pub fn get_semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }

    pub fn get_semi_minor_axis(&self) -> f64 {
        self.semi_major_axis * f64::sqrt(self.eccentricity.powi(2) - 1.0)
    }

    pub fn get_argument_of_periapsis(&self) -> f64 {
        self.argument_of_periapsis
    }

    pub fn get_min_asymptote_theta(&self) -> f64 {
        self.min_asymptote_theta
    }

    pub fn get_max_asymptote_theta(&self) -> f64 {
        self.max_asymptote_theta
    }

    pub fn get_eccentricity(&self) -> f64 {
        self.eccentricity
    }

    pub fn is_time_between_points(start: &OrbitPoint, end: &OrbitPoint, time: f64) -> bool {
        time > start.get_time() && time < end.get_time()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::components::trajectory_component::orbit::scary_math::{eccentricity, semi_major_axis, GRAVITATIONAL_CONSTANT};

    use super::*;

    #[test]
    fn test_time_from_true_anomaly_1() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(100.0);
        let time = hyperbola.get_time_since_last_periapsis(theta);
        let expected_time = 1.15 * 60.0 * 60.0;
        assert!((time - expected_time).abs() < 3.0);
    }

    #[test]
    fn test_time_from_true_anomaly_2() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(-100.0);
        let time = hyperbola.get_time_since_last_periapsis(theta);
        let expected_time = -1.15 * 60.0 * 60.0;
        assert!((time - expected_time).abs() < 3.0);
    }

    #[test]
    fn test_theta_from_time_since_periapsis_1() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let expected_theta = 0.0;
        let theta = hyperbola.get_theta_from_time_since_periapsis(0.0);
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_theta_from_time_since_periapsis_2() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let time = 4.15 * 60.0 * 60.0;
        let theta = hyperbola.get_theta_from_time_since_periapsis(time);
        let expected_theta = f64::to_radians(107.78);
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_theta_from_time_since_periapsis_3() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let time = -4.15 * 60.0 * 60.0;
        let theta = hyperbola.get_theta_from_time_since_periapsis(time);
        let expected_theta = f64::to_radians(-107.78) + 2.0*PI;
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_theta_from_time_since_periapsis_4() {
        let position = vec2(-33_839_778.563_934_326, -31_862_122.134_700_775);
        let velocity = vec2(1_187.329_620_258_232_8, 268.876_670_920_092_8);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 7.346e22;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let expected_theta = f64::atan2(position.y, position.x) + 2.0*PI;
        let time_since_periapsis = hyperbola.get_time_since_last_periapsis(expected_theta);
        let theta = hyperbola.get_theta_from_time_since_periapsis(time_since_periapsis);
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_radius_from_true_anomaly() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(107.78);
        let radius = hyperbola.get_position(theta).magnitude();
        let expected_radius = 1.632_298_46e08;
        assert!((radius - expected_radius).abs() < 1.0);
    }

    #[test]
    fn test_position_from_true_anomaly_1() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let new_position = hyperbola.get_position(0.0);
        let position_difference = new_position - position;
        assert!(position_difference.x.abs() < 0.1);
        assert!(position_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_position_from_true_anomaly_2() {
        let position = vec2(6_678_100.0 * f64::cos(PI / 4.0), 6_678_100.0 * f64::sin(PI / 4.0));
        let velocity = vec2(-15000.0 * f64::cos(PI / 4.0), 15000.0 * f64::sin(PI / 4.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let new_position = hyperbola.get_position(PI / 4.0);
        let position_difference = new_position - position;
        assert!(position_difference.x.abs() < 0.1);
        assert!(position_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_position_from_true_anomaly_3() {
        let position = vec2(-22_992_216.820_260_637, -41_211_039.677_102_46);
        let velocity = vec2(281.568_130_319_253_7, -961.589_073_059_944_4);
        let standard_gravitational_parameter = 4_902_720_400_000.0;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::atan2(position.y, position.x);
        let new_position = hyperbola.get_position(theta);
        let position_difference = new_position - position;
        assert!(position_difference.x.abs() < 0.1);
        assert!(position_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_1() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = 0.0;
        let new_velocity = hyperbola.get_velocity(hyperbola.get_position(theta), theta);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.1);
        assert!(velocity_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_2() {
        let position = vec2(6_678_100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(107.78);
        let expected_speed = 1.051_245_7e4;
        let speed = hyperbola.get_velocity(hyperbola.get_position(theta), theta).magnitude();
        assert!((speed - expected_speed).abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_3() {
        let position = vec2(0.0, 6_678_100.0);
        let velocity = vec2(-15000.0, 0.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(-107.78 + 90.0);
        let expected_speed = 1.05126e4;
        let speed = hyperbola.get_velocity(hyperbola.get_position(theta), theta).magnitude();
        assert!((speed - expected_speed).abs() < 0.5);
    }

    #[test]
    fn test_velocity_from_true_anomaly_4() {
        let position = vec2(6_678_100.0 * f64::cos(PI / 4.0), 6_678_100.0 * f64::sin(PI / 4.0));
        let velocity = vec2(-20000.0 * f64::cos(PI / 4.0), 20000.0 * f64::sin(PI / 4.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = PI / 4.0;
        let new_position = hyperbola.get_position(theta);
        let new_velocity = hyperbola.get_velocity(new_position, theta);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.1);
        assert!(velocity_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_5() {
        let position = vec2(-22_992_216.820_260_637, -4_211_039.677_102_46);
        let velocity = vec2(1_201.898_938_652_350_6, 73.283_310_932_457_88);
        let standard_gravitational_parameter = 4_902_720_400_000.0;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::atan2(position.y, position.x);
        let new_position = hyperbola.get_position(theta);
        let new_velocity = hyperbola.get_velocity(new_position, theta);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.1);
        assert!(velocity_difference.y.abs() < 0.1);
    }
}