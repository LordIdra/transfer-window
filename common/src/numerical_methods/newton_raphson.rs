#[cfg(feature = "profiling")]
use tracy_client::span;

use crate::numerical_methods::util::differentiate_2;

use super::util::differentiate_1;

pub fn newton_raphson(function: &impl Fn(f64) -> f64, starting_x: f64, max_delta: f64, max_iterations: usize) -> f64 {
    #[cfg(feature = "profiling")]
    let _span = span!("Newton-Raphson");
    let mut x = starting_x;
    let mut i = 0;
    loop {
        let (f, f_prime) = differentiate_1(function, x);
        let delta = -f/f_prime;
        x += delta;
        i += 1;
        if delta.abs() < max_delta {
            break;
        }
        assert!(i < max_iterations, "Newton-Raphson solver exceeded max iterations");

    }
    x
}

pub fn newton_raphson_to_find_stationary_point(function: &impl Fn(f64) -> f64, starting_x: f64, max_delta: f64, max_iterations: usize) -> f64 {
    #[cfg(feature = "profiling")]
    let _span = span!("Newton-Raphson to find stationary point");
    let mut x = starting_x;
    let mut i = 0;
    loop {
        #[cfg(feature = "profiling")]
        let _span = span!("Newton-Raphson iteration");
        let (_, f_prime, f_prime_prime) = differentiate_2(function, x);
        let delta = -f_prime/f_prime_prime;
        x += delta;
        i += 1;
        if delta.abs() < max_delta {
            break;
        }
        assert!(i < max_iterations, "Newton-Raphson solver exceeded max iterations");
    }
    x
}

#[cfg(test)]
mod test {
    use crate::numerical_methods::newton_raphson::{newton_raphson, newton_raphson_to_find_stationary_point};

    #[test]
    fn test_newton_raphson() {
        let function = |x: f64| x.powi(2) - 4.0;
        let starting_x = 4.0;
        assert!((newton_raphson(&function, starting_x, 1.0e-6, 20) - 2.0).abs() < 1.0e-3);
    }

    #[test]
    fn test_newton_raphson_to_find_stationary_point() {
        let function = |x: f64| x.powi(2) - 4.0*x - 10.0;
        let starting_x = -1.74;
        assert!((newton_raphson_to_find_stationary_point(&function, starting_x, 1.0e-6, 20) - 2.0).abs() < 1.0e-3);
    }
}