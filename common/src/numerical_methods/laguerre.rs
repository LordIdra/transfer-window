#[cfg(feature = "profiling")]
use tracy_client::span;

use crate::numerical_methods::util::{differentiate_2, differentiate_3};

fn delta(f: f64, f_prime: f64, f_prime_prime: f64) -> f64 {
    if f == 0.0 {
        // Yes, this is actually a scenario we have to worry about...
        return 0.0;
    }
    let n: f64 = 2.0; // N=2, AKA modified Newton-Raphson
    let g = f_prime / f;
    let h = g.powi(2) - f_prime_prime / f;
    let c = (n - 1.0) * (n * h - g.powi(2));
    let b = c.abs().sqrt() * g.signum();
    -n / (g + b)
}

pub fn laguerre(function: &impl Fn(f64) -> f64, starting_x: f64, max_delta: f64, max_iterations: usize) -> f64 {
    #[cfg(feature = "profiling")]
    let _span = span!("Laguerre");
    let mut x = starting_x;
    let mut i = 0;
    loop {
        let (f, f_prime, f_prime_prime) = differentiate_2(function, x);
        let delta = delta(f, f_prime, f_prime_prime);
        x += delta;
        i += 1;
        if delta.abs() < max_delta {
            break;
        }
        assert!(i < max_iterations, "Laguerre solver exceeded max iterations");
    }
    x
}

pub fn laguerre_to_find_stationary_point(function: &impl Fn(f64) -> f64, starting_x: f64, max_delta: f64, max_iterations: usize) -> f64 {
    #[cfg(feature = "profiling")]
    let _span = span!("Laguerre to find stationary point");
    let mut x = starting_x;
    let mut i = 0;
    loop {
        let (_, f_prime, f_prime_prime, f_prime_prime_prime) = differentiate_3(function, x);
        let delta = delta(f_prime, f_prime_prime, f_prime_prime_prime);
        x += delta;
        i += 1;
        if delta.abs() < max_delta {
            break;
        }
        assert!(i < max_iterations, "Laguerre solver exceeded max iterations");
    }
    x
}

#[cfg(test)]
mod test {
    use crate::numerical_methods::laguerre::{laguerre, laguerre_to_find_stationary_point};



    #[test]
    fn test_laguerre() {
        let function = |x: f64| x.powi(2) - 4.0;
        let starting_x = 4.0;
        assert!((laguerre(&function, starting_x, 1.0e-6, 20) - 2.0).abs() < 1.0e-3);
    }

    #[test]
    fn test_laguerre_raphson_to_find_stationary_point() {
        let function = |x: f64| x.powi(2) - 4.0*x - 10.0;
        let starting_x = -1.74;
        assert!((laguerre_to_find_stationary_point(&function, starting_x, 1.0e-6, 20) - 2.0).abs() < 1.0e-3);
    }
}