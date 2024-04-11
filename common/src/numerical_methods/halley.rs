use crate::numerical_methods::util::{differentiate_2, differentiate_3};

fn delta(f: f64, f_prime: f64, f_prime_prime: f64) -> f64 {
    -2.0 * f * f_prime / (2.0 * f_prime.powi(2) - f * f_prime_prime)
}

pub fn halley(function: &impl Fn(f64) -> f64, starting_x: f64, derivative_delta: f64, max_delta: f64, max_iterations: usize) -> Option<f64> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Halley");
    let mut x = starting_x;
    let mut i = 0;
    loop {
        let (f, f_prime, f_prime_prime) = differentiate_2(function, x, derivative_delta);
        let delta = delta(f, f_prime, f_prime_prime);
        x += delta;
        i += 1;
        if delta.abs() < max_delta {
            break;
        }
        if i > max_iterations {
            return None;
        }

    }
    Some(x)
}

pub fn halley_to_find_stationary_point(function: &impl Fn(f64) -> f64, starting_x: f64, derivative_delta: f64, max_delta: f64, max_iterations: usize) -> Option<f64> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Halley to find stationary point");
    let mut x = starting_x;
    let mut i = 0;
    loop {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Halley iteration");
        let (_, f_prime, f_prime_prime, f_prime_prime_prime) = differentiate_3(function, x, derivative_delta);
        let delta = delta(f_prime, f_prime_prime, f_prime_prime_prime);
        x += delta;
        i += 1;
        if delta.abs() < max_delta {
            break;
        }
        if i > max_iterations {
            return None;
        }
    }
    Some(x)
}

#[cfg(test)]
mod test {
    use crate::numerical_methods::halley::{halley, halley_to_find_stationary_point};


    #[test]
    fn test_halley() {
        let function = |x: f64| x.powi(2) - 4.0;
        let starting_x = 4.0;
        dbg!(halley(&function, starting_x, 1.0e-6, 1.0e-6, 50).unwrap() - 2.0);
        assert!((halley(&function, starting_x, 1.0e-6, 1.0e-6, 50).unwrap().abs() - 2.0).abs() < 1.0e-3);
    }

    #[test]
    fn test_halley_to_find_stationary_point() {
        let function = |x: f64| x.powi(2) - 4.0*x - 10.0;
        let starting_x = -1.74;
        assert!((halley_to_find_stationary_point(&function, starting_x, 1.0e-6, 1.0e-6, 50).unwrap().abs() - 2.0).abs() < 1.0e-3);
    }
}