use kurbo::common::solve_itp;

const MAX_DELTA: f64 = 1.0e-6;

/// # Panics
/// Panics if f(min) >= 0 or f(max) <= 0
/// # Errors
/// Errors if f(min) > 0 or f(max) < 0
pub fn itp(f: &impl Fn(f64) -> f64, min: f64, max: f64) -> Result<f64, &'static str> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("ITP solver");
    if !(f(min).is_sign_negative() && f(max).is_sign_positive()) {
        return Err("ITP assertion that f(min) < 0 and f(max) > 0 failed");
    }
    if min < max {
        Ok(solve_itp(f, min, max, MAX_DELTA, 1, 0.2 / (max - min).abs(), f(min), f(max)))
    } else {
        let f = |x: f64| -f(x);
        Ok(solve_itp(f, max, min, MAX_DELTA, 1, 0.2 / (max - min).abs(), f(max), f(min)))
    }
}