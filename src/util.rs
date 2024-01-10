use std::f64::consts::PI;

/// Normalizes between 0 and 2pi
pub fn normalize_angle(theta: f64) -> f64 {
    theta.signum() * (theta % (2.0 * PI))
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::util::normalize_angle;

    #[test]
    fn test_normalize_angle() {
        assert!((normalize_angle(2.01 * PI) - 0.01 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(3.789 * PI) - 1.789 * PI).abs() < 1.0e-5);
        assert!((normalize_angle(-1.345 * PI) - 1.345 * PI).abs() < 1.0e-5);
    }
}