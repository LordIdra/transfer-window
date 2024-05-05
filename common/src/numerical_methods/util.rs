/// Returns (value, first derivative)
pub fn differentiate_1(f: &impl Fn(f64) -> f64, x: f64, delta: f64) -> (f64, f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Differentiate 1");
    let f_1 = f(x - delta);
    let f_2 = f(x);

    (f_2, (f_2 - f_1) / delta)
}

/// Returns (value, first derivative, second derivative)
pub fn differentiate_2(f: &impl Fn(f64) -> f64, x: f64, delta: f64) -> (f64, f64, f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Differentiate 2");
    let f_1 = f(x);
    let f_2 = f(x + delta);
    let f_3 = f(x + 2.0 * delta);
    
    let f_prime_1 = (f_2 - f_1) / delta;
    let f_prime_2 = (f_3 - f_2) / delta;

    let f_prime_prime = (f_prime_2 - f_prime_1) / delta;

    (f_2, f_prime_1, f_prime_prime)
}


/// Returns (value, first derivative, second derivative, third derivative)
pub fn differentiate_3(f: &impl Fn(f64) -> f64, x: f64, delta: f64) -> (f64, f64, f64, f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Differentiate 3");
    let f_1 = f(x - delta);
    let f_2 = f(x);
    let f_3 = f(x + delta);
    let f_4 = f(x + 2.0 * delta);

    let f_prime_1 = (f_2 - f_1) / delta;
    let f_prime_2 = (f_3 - f_2) / delta;
    let f_prime_3 = (f_4 - f_3) / delta;

    let f_prime_prime_1 = (f_prime_2 - f_prime_1) / delta;
    let f_prime_prime_2 = (f_prime_3 - f_prime_2) / delta;

    let f_prime_prime_prime = (f_prime_prime_2 - f_prime_prime_1) / delta;

    (f_2, f_prime_1, f_prime_prime_1, f_prime_prime_prime)
}
