use std::f64::consts::PI;

use nalgebra_glm::{DVec2, vec2};

use super::orbit_direction::OrbitDirection;

pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;
pub const STANDARD_GRAVITY: f64 = 9.81;

// https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector#mjx-eqn-9.5.31
// https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/elliptical-orbits.html

/// Returns Component of velocity perpendicular to the displacement
pub fn transverse_velocity(position: DVec2, velocity: DVec2) -> f64 {
    (velocity.y * position.x - velocity.x * position.y) / position.magnitude()
}

pub fn semi_major_axis(position: DVec2, velocity: DVec2, standard_gravitational_parameter: f64) -> f64 {
    ((2.0 / position.magnitude()) - (velocity.magnitude().powi(2) / standard_gravitational_parameter)).powi(-1)
}

pub fn eccentricity(position: DVec2, velocity: DVec2, standard_gravitational_parameter: f64, semi_major_axis: f64) -> f64 {
    (1.0 - ((position.magnitude_squared() * transverse_velocity(position, velocity).powi(2)) / (standard_gravitational_parameter * semi_major_axis))).sqrt()
}

pub fn speed_to_obtain_eccentricity(position: DVec2, eccentricity: f64, standard_gravitational_parameter: f64, semi_major_axis: f64) -> f64 {
    f64::sqrt(standard_gravitational_parameter * semi_major_axis * (1.0 - eccentricity) / position.magnitude_squared())
}

pub fn velocity_to_obtain_eccentricity(position: DVec2, eccentricity: f64, standard_gravitational_parameter: f64, semi_major_axis: f64, direction: OrbitDirection) -> DVec2 {
    let speed = speed_to_obtain_eccentricity(position, eccentricity, standard_gravitational_parameter, semi_major_axis);
    let mut velocity_unit = vec2(-position.y, position.x).normalize();
    if direction.is_clockwise() {
        velocity_unit = -velocity_unit;
    }
    velocity_unit * speed
}

pub fn argument_of_periapsis(position: DVec2, velocity: DVec2, standard_gravitational_parameter: f64) -> f64 {
    let eccentricity_vector = ((velocity.magnitude().powi(2) - standard_gravitational_parameter / position.magnitude()) * position - (position.dot(&velocity) * velocity)) / standard_gravitational_parameter;
    f64::atan2(eccentricity_vector.y, eccentricity_vector.x)
}

pub fn specific_angular_momentum(position: DVec2, velocity: DVec2) -> f64 {
    position.magnitude() * transverse_velocity(position, velocity)
}

pub fn period(standard_gravitational_parameter: f64, semi_major_axis: f64) -> f64 {
    2.0 * PI * f64::sqrt(semi_major_axis.powi(3) / standard_gravitational_parameter)
}

pub fn sphere_of_influence(mass: f64, parent_mass: f64, position: DVec2, velocity: DVec2) -> f64 {
    let semi_major_axis = semi_major_axis(position, velocity, GRAVITATIONAL_CONSTANT * parent_mass);
    semi_major_axis * (mass / parent_mass).powf(2.0 / 5.0)
}

pub fn asymptote_theta(eccentricity: f64, argument_of_periapsis: f64) -> (f64, f64) {
    let true_anomaly_of_asymptote = f64::acos(-1.0 / eccentricity);
    (argument_of_periapsis - true_anomaly_of_asymptote, argument_of_periapsis + true_anomaly_of_asymptote)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    use nalgebra_glm::vec2;

    #[test]
    fn test_semi_major_axis() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10 * f64::cos(PI / 6.0),  6.9818e10 * f64::sin(PI / 6.0));
        let velocity = vec2(3.886e4 * f64::cos(PI / 6.0 + PI / 2.0), 3.886e4 * f64::sin(PI / 6.0 + PI / 2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.989e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        // actual SMA is slightly different due to N-body perturbations and the like
        // yes this is 'slightly' in astronomical terms
        assert!((semi_major_axis - 5.7909e10).abs() < 1.0e7); 
    }

    #[test]
    fn test_eccentricity_1() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10 * f64::cos(-PI / 6.0), 6.9818e10 * f64::sin(-PI / 6.0),);
        let velocity = vec2(3.886e4 * f64::cos(-PI / 6.0 + PI / 2.0), 3.886e4 * f64::sin(-PI / 6.0 + PI / 2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.989e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        assert!((eccentricity - 0.2056).abs() < 0.001);
    }

    #[test]
    fn test_eccentricity_2() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(6_678_100.0 * f64::cos(-PI / 6.0), 6_678_100.0 * f64::sin(-PI / 6.0));
        let velocity = vec2(15000.0 * f64::cos(-PI / 6.0 + PI / 2.0), 15000.0 * f64::sin(-PI / 6.0 + PI / 2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        assert!((eccentricity - 2.7696).abs() < 0.001);
    }

    #[test]
    fn test_argument_of_periapsis_1() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10 * f64::cos(-PI / 6.0), 6.9818e10 * f64::sin(-PI / 6.0),);
        let velocity = vec2(-3.886e4 * f64::cos(-PI / 6.0 + PI / 2.0), -3.886e4 * f64::sin(-PI / 6.0 + PI / 2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.9895e30;
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let expected_argument_of_periapsis = -PI / 6.0 + PI;
        assert!((argument_of_periapsis - expected_argument_of_periapsis).abs() < 0.01);
    }

    #[test]
    fn test_argument_of_periapsis_2() {
        let position = vec2(0.4055e9 * f64::cos(PI/6.0), 0.4055e9 * f64::sin(PI/6.0));
        let velocity = vec2(0.570e3 * f64::cos(PI/6.0 + PI/2.0), 0.570e3 * f64::sin(PI/6.0 + PI/2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let expected_argument_of_periapsis = PI / 6.0 - PI;
        assert!((argument_of_periapsis - expected_argument_of_periapsis).abs() < 0.01);
    }

    #[test]
    fn test_argument_of_periapsis_3() {
        let position = vec2(369_236_029.358_813_2, 143_598_629.719_664_34);
        let velocity = vec2(47.799_689_595_602_02, -607.392_053_430_677_3);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let expected_argument_of_periapsis = PI / 6.0 - PI;
        assert!((argument_of_periapsis - expected_argument_of_periapsis).abs() < 0.01);
    }

    #[test]
    fn test_argument_of_periapsis_4() {
        let position = vec2(221_244_867.958_108_5, 278_127_601.097_456_3);
        let velocity = vec2(772.330_351_134_78, -73.803_348_907_595_99);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let expected_argument_of_periapsis = PI / 6.0 - PI;
        assert!((argument_of_periapsis - expected_argument_of_periapsis).abs() < 0.01);
    }

    #[test]
    fn test_argument_of_periapsis_5() {
        let position = vec2(321_699_434.075_753_2, 238_177_462.813_335_57);
        let velocity = vec2(-448.885_375_943_825_5, 386.138_758_435_720_83);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let expected_argument_of_periapsis = -2.615_930_001_576_588;
        assert!((argument_of_periapsis - expected_argument_of_periapsis).abs() < 0.01);
    }

    #[test]
    fn test_period_1() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
        let position = vec2(1.52100e11, 0.0);
        let velocity = vec2(0.0, 2.929e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.9895e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let period = period(standard_gravitational_parameter, semi_major_axis) / (60.0 * 60.0 * 24.0);
        let expected_period = 364.9;
        assert!((period - expected_period).abs() < 0.1);
    }

    #[test]
    fn test_period_2() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10, 0.0);
        let velocity = vec2(0.0, 3.886e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.9895e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let period = period(standard_gravitational_parameter, semi_major_axis) / (60.0 * 60.0 * 24.0);
        let expected_period = 87.969;
        assert!((period - expected_period).abs() < 0.1);
    }

    #[test]
    fn test_period_3() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10 * f64::cos(-PI / 6.0), 6.9818e10 * f64::sin(-PI / 6.0));
        let velocity = vec2(3.886e4 * f64::cos(-PI / 6.0 + PI / 2.0), 3.886e4 * f64::sin(-PI / 6.0 + PI / 2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.9895e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let period = period(standard_gravitational_parameter, semi_major_axis) / (60.0 * 60.0 * 24.0);
        let expected_period = 87.969;
        assert!((period - expected_period).abs() < 0.1);
    }
}