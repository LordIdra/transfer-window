use std::f64::consts::PI;

use transfer_window_common::numerical_methods::{itp::itp, laguerre::laguerre_to_find_stationary_point};

use crate::{components::path_component::orbit::Orbit, storage::entity_allocator::Entity, api::trajectories::fast_solver::bounding::util::{angle_window_to_time_window, angular_distance, make_range_containing}, util::normalize_angle};

use super::{sdf::make_sdf, util::find_other_stationary_point, window::Window};

fn find_intersections(f: &impl Fn(f64) -> f64, min_theta: f64, max_theta: f64) -> Result<(f64, f64), &'static str> {
    let theta_1 = itp(&f, min_theta, max_theta)?;
    // the other angle is in the 'opposite' range
    // we can find this by subtracting 2pi from the highest theta
    let (new_min_theta, new_max_theta) = if min_theta > max_theta {
        (min_theta - 2.0 * PI, max_theta)
    } else {
        (min_theta, max_theta - 2.0 * PI)
    };
    let theta_2 = itp(&f, new_min_theta, new_max_theta)?;
    let theta_2 = normalize_angle(theta_2);
    Ok((theta_1, theta_2))
}

fn find_min_max_signed_distance(sdf: &impl Fn(f64) -> f64, argument_of_apoapsis: f64) -> Result<(f64, f64), &'static str> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Min max signed distance");
    let theta_1 = laguerre_to_find_stationary_point(&sdf, argument_of_apoapsis, 1.0e-4, 1.0e-6, 256).expect("Failed to converge while solving for ellipse bounds");
    let theta_2 = find_other_stationary_point(sdf, theta_1)?;
    let (theta_1, theta_2) = (normalize_angle(theta_1), normalize_angle(theta_2));
    if sdf(theta_1) < sdf(theta_2) { 
        Ok((theta_1, theta_2))
    } else { 
        Ok((theta_2, theta_1))
    }
}

struct BounderData<'a> {
    orbit: &'a Orbit, 
    sibling_orbit: &'a Orbit, 
    sibling: Entity, 
    min_theta: f64, 
    max_theta: f64, 
    soi: f64,
    start_time: f64,
}

impl<'a> BounderData<'a> {
    fn make_segmented_window(&self, periodic: bool, from: f64, to: f64, segments: usize) -> Vec<Window<'a>> {
        let mut windows = vec![];
        for i in 0..segments {
            let start = from + (i as f64 / segments as f64) * (to - from);
            let end = from + ((i + 1) as f64 / segments as f64) * (to - from);
            let window = Window::new(self.orbit, self.sibling_orbit, self.sibling, periodic, (start, end));
            windows.push(window);
        }
        windows
    }

    fn no_bounds(self) -> Vec<Window<'a>> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("No bounds");
        // Well, this is awkward, the orbit is ALWAYS within the sibling's SOI
        // We split the orbit up into many segments
        // It's incredibly unlikely that a segment will contain multiple minimums, 
        // and all we need is a segment that contains 0-1 minimums for the solver
        self.make_segmented_window(true, self.start_time, self.start_time + self.orbit.period().unwrap(), 16)
    }

    fn no_encounters() -> Vec<Window<'a>> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("No encounters");
        vec![]
    }

    fn one_bound_inner(self, sdf: impl Fn(f64) -> f64) -> Result<Vec<Window<'a>>, &'static str> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("One bound inner");
        // Bound start/end points are on the INSIDE
        let f = |theta: f64| (sdf)(theta) - self.soi;
        let intersections = find_intersections(&f, self.min_theta, self.max_theta)?;
        let angle_bound = make_range_containing(intersections.0, intersections.1, self.min_theta);
        let bound = angle_window_to_time_window(self.orbit, angle_bound);
        // The reason we segment here is that our solver relies on the assumption that if there is a minimum,
        // the derivative with respect to time is positive at the start and negative at the end
        // There are some cases where this is not true, but splitting the window up into a few segments helps keep this assumption
        Ok(self.make_segmented_window(true, bound.0, bound.1, 4))
    }

    fn one_bound_outer(self, sdf: impl Fn(f64) -> f64) -> Result<Vec<Window<'a>>, &'static str> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("One bound outer");
        // Bound start/end points are on the OUTSIDE
        let f = |theta: f64| (sdf)(theta) + self.soi;
        let intersections = find_intersections(&f, self.min_theta, self.max_theta)?;
        let angle_bound = make_range_containing(intersections.0, intersections.1, self.max_theta);
        let bound = angle_window_to_time_window(self.orbit, angle_bound);
        // The reason we segment here is that our solver relies on the assumption that if there is a minimum,
        // the derivative with respect to time is positive at the start and negative at the end
        // There are some cases where this is not true, but splitting the window up into a few segments helps keep this assumption
        Ok(self.make_segmented_window(true, bound.0, bound.1, 4))
    }

    fn two_bounds(self, sdf: impl Fn(f64) -> f64) -> Result<Vec<Window<'a>>, &'static str> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Two bounds");
        let f_inner = |theta: f64| (sdf)(theta) - self.soi;
        let f_outer = |theta: f64| (sdf)(theta) + self.soi;
        #[cfg(feature = "profiling")]
        let _span1 = tracy_client::span!("Finding intersections");
        let inner_intersections = find_intersections(&f_inner, self.min_theta, self.max_theta)?;
        let outer_intersections = find_intersections(&f_outer, self.min_theta, self.max_theta)?;
        let zero_intersections = find_intersections(&sdf, self.min_theta, self.max_theta)?;
        #[cfg(feature = "profiling")]
        drop(_span1);

        // We have 4 points, and know where the orbits intersect
        // Now we need to create two windows that cover exactly one intersection
        let from = inner_intersections.0;
        let mut possible_tos = vec![inner_intersections.1, outer_intersections.0, outer_intersections.1];
        let mut to_index = 0;
        let mut min_distance = f64::MAX;
        for (i, possible_to) in possible_tos.iter().enumerate() {
            let window = make_range_containing(from, *possible_to, zero_intersections.0);
            let distance = angular_distance(window.0, window.1);
            if distance < min_distance {
                min_distance = distance;
                to_index = i;
            }
        }
        let to = possible_tos.remove(to_index);
        let angle_bound_1 = make_range_containing(from, to, zero_intersections.0);
        let angle_bound_2 = make_range_containing(possible_tos[0], possible_tos[1], zero_intersections.1);

        let bound_1 = angle_window_to_time_window(self.orbit, angle_bound_1);
        let bound_2 = angle_window_to_time_window(self.orbit, angle_bound_2);

        let window_1 = Window::new(self.orbit, self.sibling_orbit, self.sibling, true, bound_1);
        let window_2 = Window::new(self.orbit, self.sibling_orbit, self.sibling, true, bound_2);

        Ok(vec![window_1, window_2] )
    }
}

pub fn compute_ellipse_bound<'a>(orbit: &'a Orbit, sibling_orbit: &'a Orbit, sibling: Entity, start_time: f64) -> Result<Vec<Window<'a>>, &'static str> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Ellipse bounding");
    let argument_of_apoapsis = orbit.argument_of_periapsis() + PI;
    let sdf = make_sdf(orbit, sibling_orbit);
    let (min_theta, max_theta) = find_min_max_signed_distance(&sdf, argument_of_apoapsis)?;
    let (min, max) = (sdf(min_theta), sdf(max_theta));
    let soi = sibling_orbit.sphere_of_influence();
    let data = BounderData {
        orbit,
        sibling_orbit,
        sibling,
        min_theta,
        max_theta,
        soi,
        start_time,
    };

    if min.abs() < soi && max.abs() < soi {
        Ok(data.no_bounds())
    } else if (max.is_sign_positive() && min.is_sign_positive() && min.abs() > soi) 
           || (max.is_sign_negative() && min.is_sign_negative() && max.abs() > soi) {
        Ok(BounderData::no_encounters())
    } else if (max.is_sign_positive() && min.is_sign_positive() && min.abs() < soi)
           || (max.is_sign_positive() && min.is_sign_negative() && min.abs() < soi) {
        data.one_bound_inner(sdf)
    } else if (max.is_sign_positive() && min.is_sign_negative() && max.abs() < soi)
           || (max.is_sign_negative() && min.is_sign_negative() && max.abs() < soi) {
        data.one_bound_outer(sdf)
    } else {
        data.two_bounds(sdf)
    }
}
