use crate::constants::{NEWTON_SOLVER_DERIVATIVE_DELTA, NEWTON_SOLVER_MAX_DELTA, NEWTON_SOLVER_MAX_ITERATIONS, NEWTON_SOLVER_MIN_VALUE};

/// Returns (first derivative, second derivative)
fn differentiate(f: &impl Fn(f64) -> f64, x: f64) -> (f64, f64) {
    let f_1 = f(x - NEWTON_SOLVER_DERIVATIVE_DELTA);
    let f_2 = f(x);
    let f_3 = f(x + NEWTON_SOLVER_DERIVATIVE_DELTA);

    let f_prime_1 = (f_2 - f_1) / NEWTON_SOLVER_DERIVATIVE_DELTA;
    let f_prime_2 = (f_3 - f_2) / NEWTON_SOLVER_DERIVATIVE_DELTA;
    let f_prime = (f_prime_1 + f_prime_2) / 2.0;
    let f_prime_prime = (f_prime_2 - f_prime_1) / NEWTON_SOLVER_DERIVATIVE_DELTA;
    (f_prime, f_prime_prime)
}

pub fn newton_raphson_to_find_stationary_point(function: &impl Fn(f64) -> f64, starting_x: f64) -> Option<f64> {
    let mut x = starting_x;
    let mut i = 0;
    while i < NEWTON_SOLVER_MAX_ITERATIONS {
        if x < NEWTON_SOLVER_MIN_VALUE {
            return None;
        }
        let (first, second) = differentiate(function, x);
        let delta = -first/second;
        if delta.abs() < NEWTON_SOLVER_MAX_DELTA {
            return Some(x);
        }
        x += delta;
        i += 1;
    }
    return None;
}

pub fn newton_raphson(function: &impl Fn(f64) -> f64, starting_x: f64) -> Option<f64> {
    let mut x = starting_x;
    let mut i = 0;
    while i < NEWTON_SOLVER_MAX_ITERATIONS {
        if x < NEWTON_SOLVER_MIN_VALUE {
            return None;
        }
        let f_1 = function(x);
        let f_2 = function(x + NEWTON_SOLVER_DERIVATIVE_DELTA);
        let derivative = (f_2 - f_1) / NEWTON_SOLVER_DERIVATIVE_DELTA;
        let delta = -f_1 / derivative;
        if delta.abs() < NEWTON_SOLVER_MAX_DELTA {
            return Some(x);
        }
        x += delta;
        i += 1;
    }
    return None;
}

#[cfg(test)]
mod test {
    use crate::systems::trajectory_prediction::numerical_methods::newton_raphson::{newton_raphson, newton_raphson_to_find_stationary_point};

    #[test]
    fn test_newton_raphson() {
        let function = |x: f64| x.powi(2) - 4.0;
        let starting_x = 4.0;
        assert!((newton_raphson(&function, starting_x).unwrap() - 2.0).abs() < 1.0e-3);
    }

    #[test]
    fn test_newton_raphson_to_find_stationary_point() {
        let function = |x: f64| x.powi(2) - 4.0*x - 10.0;
        let starting_x = -1.74;
        assert!((newton_raphson_to_find_stationary_point(&function, starting_x).unwrap() - 2.0).abs() < 1.0e-3);
    }
}