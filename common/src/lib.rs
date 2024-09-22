use std::f64::consts::PI;

pub mod numerical_methods;

/// Normalizes between 0 and 2pi
/// <https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation>
pub fn normalize_angle(mut theta: f64) -> f64 {
    theta %= 2.0 * PI;
    (theta + 2.0 * PI) % (2.0 * PI)
}

/// Returns the (smallest) distance from the first angle to the second going anticlockwise, ie wrapping round if necessary
pub fn anticlockwise_angular_distance(from: f64, to: f64) -> f64 {
    let from = normalize_angle(from);
    let to = normalize_angle(to);
    if from < to {
        to - from
    } else {
        to + 2.0*PI - from
    }
}

/// Returns the smallest distance from the angle to the second, either clockwise or anticlockwise
pub fn angular_distance(from: f64, to: f64) -> f64 {
    let distance = anticlockwise_angular_distance(from, to);
    if distance > PI {
        distance - 2.0 * PI
    } else {
        distance
    }
}


#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::{normalize_angle, angular_distance};

    #[test]
    fn test_normalize_angle() {
        assert!((normalize_angle(2.01 * PI) - 0.01 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(3.789 * PI) - 1.789 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(-1.345 * PI) - 0.655 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(-6.0 * PI + 0.2) - 0.2).abs() < 1.0e-5);
    }

    #[test]
    fn test_signed_angular_distance() {
        assert!((angular_distance(0.1, 0.3) - 0.2).abs() < 1.0e-5);
        assert!((angular_distance(0.1, -0.3) + 0.4).abs() < 1.0e-5);
        assert!((angular_distance(3.0, -3.0) + 6.0 - 2.0 * PI).abs() < 1.0e-5);
        assert!((angular_distance(-2.5, 2.0) - 4.5 + 2.0 * PI).abs() < 1.0e-5);
    }
}
