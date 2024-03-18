#[cfg(feature = "profiling")]
use tracy_client::span;

const DERIVATIVE_DELTA: f64 = 1.0e-4;

/// Returns (value, first derivative)
pub fn differentiate_1(f: &impl Fn(f64) -> f64, x: f64) -> (f64, f64) {
    #[cfg(feature = "profiling")]
    let _span = span!("Differentiate 1");
    let f_1 = f(x - DERIVATIVE_DELTA);
    let f_2 = f(x);

    (f_2, (f_2 - f_1) / DERIVATIVE_DELTA)
}

/// Returns (value, first derivative, second derivative)
pub fn differentiate_2(f: &impl Fn(f64) -> f64, x: f64) -> (f64, f64, f64) {
    #[cfg(feature = "profiling")]
    let _span = span!("Differentiate 2");
    let f_1 = f(x - DERIVATIVE_DELTA);
    let f_2 = f(x);
    let f_3 = f(x + DERIVATIVE_DELTA);

    let f_prime_1 = (f_2 - f_1) / DERIVATIVE_DELTA;
    let f_prime_2 = (f_3 - f_2) / DERIVATIVE_DELTA;

    let f_prime_prime = (f_prime_2 - f_prime_1) / DERIVATIVE_DELTA;

    (f_2, f_prime_1, f_prime_prime)
}


/// Returns (value, first derivative, second derivative, third derivative)
pub fn differentiate_3(f: &impl Fn(f64) -> f64, x: f64) -> (f64, f64, f64, f64) {
    #[cfg(feature = "profiling")]
    let _span = span!("Differentiate 3");
    let f_1 = f(x - DERIVATIVE_DELTA);
    let f_2 = f(x);
    let f_3 = f(x + DERIVATIVE_DELTA);
    let f_4 = f(x + 2.0 * DERIVATIVE_DELTA);

    let f_prime_1 = (f_2 - f_1) / DERIVATIVE_DELTA;
    let f_prime_2 = (f_3 - f_2) / DERIVATIVE_DELTA;
    let f_prime_3 = (f_4 - f_3) / DERIVATIVE_DELTA;

    let f_prime_prime_1 = (f_prime_2 - f_prime_1) / DERIVATIVE_DELTA;
    let f_prime_prime_2 = (f_prime_3 - f_prime_2) / DERIVATIVE_DELTA;

    let f_prime_prime_prime = (f_prime_prime_2 - f_prime_prime_1) / DERIVATIVE_DELTA;

    (f_2, f_prime_1, f_prime_prime_1, f_prime_prime_prime)
}