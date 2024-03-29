#[cfg(feature = "profiling")]
use tracy_client::span;

/// Terminates when solution interval is lower than `max_interval`
/// Panics when `max_iterations` is exceeded
pub fn bisection(function: &impl Fn(f64) -> f64, min: f64, max: f64, max_interval: f64, max_iterations: usize) -> f64 {
    #[cfg(feature = "profiling")]
    let _span = span!("Bisection");
    let mut low = min;
    let mut high = max;
    let mut mid = (min + max) / 2.0;
    let mut i = 0;
    loop {
        if function(mid).is_sign_positive() && function(low).is_sign_positive() || function(mid).is_sign_negative() && function(low).is_sign_negative() {
            low = mid;
        } else {
            high = mid;
        }
        mid = (low + high) / 2.0;
        i += 1;
        if (low - high).abs() < max_interval {
            break;
        }
        assert!(i < max_iterations, "Bisection solver exceeded max iterations");
    }
    mid
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::systems::trajectory_prediction::numerical_methods::bisection::bisection;

    #[test]
    fn test_bisection_1() {
        let function = |x: f64| f64::sin(x);
        assert!(bisection(&function, -PI/2.0, PI/2.0, 1.0e-3, 12).abs() < 1.0e-2);
    }

    #[test]
    fn test_bisection_2() {
        let function = |x: f64| x.powi(2) - 4.0;
        assert!(bisection(&function, 0.0,  10.0, 1.0e-3, 24).abs() - 2.0 < 1.0e-2);
        assert!(bisection(&function, -10.0, 0.0, 1.0e-3, 24).abs() - 2.0 < 1.0e-2);
    }
}