use std::f64::consts::PI;

/// Normalizes between 0 and 2pi
/// https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
pub fn normalize_angle(mut theta: f64) -> f64 {
    theta = theta % (2.0 * PI);
    (theta + 2.0 * PI) % (2.0 * PI)
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::util::normalize_angle;

    #[test]
    fn test_normalize_angle() {
        assert!((normalize_angle(2.01 * PI) - 0.01 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(3.789 * PI) - 1.789 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(-1.345 * PI) - 0.655 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(-6.0 * PI + 0.2) - 0.2).abs() < 1.0e-5);
    }
}