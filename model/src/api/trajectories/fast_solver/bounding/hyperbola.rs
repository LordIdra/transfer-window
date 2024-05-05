use crate::{components::path_component::orbit::Orbit, storage::entity_allocator::Entity, api::trajectories::fast_solver::bounding::util::{angle_window_to_time_window, make_range_containing}};

use super::{sdf::make_sdf, window::Window};

use transfer_window_common::numerical_methods::{itp::itp, util::differentiate_1};

// Hyperbolic SDF only has a maximum, no minimum
fn find_max(sdf: &impl Fn(f64) -> f64, min_asymptote_theta: f64, max_asymptote_theta: f64) -> f64 {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Max signed distance");
    let min_derivative_theta = max_asymptote_theta;
    let max_derivative_theta = min_asymptote_theta;
    let f = |theta: f64| differentiate_1(&sdf, theta, 1.0e-4).1;
    itp(&f, min_derivative_theta, max_derivative_theta)
}

struct BounderData<'a> {
    orbit: &'a Orbit, 
    sibling_orbit: &'a Orbit, 
    sibling: Entity, 
    max_theta: f64,
    min_asymptote_theta: f64,
    max_asymptote_theta: f64,
    soi: f64,
}

impl<'a> BounderData<'a> {
    fn no_encounters() -> Vec<Window<'a>> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("No encounters");
        vec![]
    }

    fn one_bound(self, sdf: impl Fn(f64) -> f64) -> Vec<Window<'a>> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("One bound");
        let f = |theta: f64| (sdf)(theta) + self.soi;
        let theta_1 = itp(&f, self.min_asymptote_theta, self.max_theta);
        let theta_2 = itp(&f, self.max_asymptote_theta, self.max_theta);
        let angle_bound = make_range_containing(theta_1, theta_2, self.max_theta);
        let bound = angle_window_to_time_window(self.orbit, angle_bound);
        let window = Window::new(self.orbit, self.sibling_orbit, self.sibling, false, bound);
        vec![window]
    }

    fn two_bounds(self, sdf: impl Fn(f64) -> f64) -> Vec<Window<'a>> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Two bounds");
        let f_inner = |theta: f64| (sdf)(theta) + self.soi;
        let f_outer = |theta: f64| (sdf)(theta) - self.soi;

        let theta_1 = itp(&sdf, self.min_asymptote_theta, self.max_theta);
        let theta_1_outer = itp(&f_outer, self.min_asymptote_theta, self.max_theta);
        let theta_1_inner = itp(&f_inner, self.min_asymptote_theta, self.max_theta);

        let theta_2 = itp(&sdf, self.max_asymptote_theta, self.max_theta);
        let theta_2_outer = itp(&f_outer, self.max_asymptote_theta, self.max_theta);
        let theta_2_inner = itp(&f_inner, self.max_asymptote_theta, self.max_theta);

        let angle_bound_1 = make_range_containing(theta_1_inner, theta_1_outer, theta_1);
        let angle_bound_2 = make_range_containing(theta_2_inner, theta_2_outer, theta_2);

        let bound_1 = angle_window_to_time_window(self.orbit, angle_bound_1);
        let bound_2 = angle_window_to_time_window(self.orbit, angle_bound_2);

        let window_1 = Window::new(self.orbit, self.sibling_orbit, self.sibling, false, bound_1);
        let window_2 = Window::new(self.orbit, self.sibling_orbit, self.sibling, false, bound_2);

        vec![window_1, window_2]
    }
}

pub fn compute_hyperbola_bound<'a>(orbit: &'a Orbit, sibling_orbit: &'a Orbit, sibling: Entity) -> Vec<Window<'a>> {
    let sdf = make_sdf(orbit, sibling_orbit);
    let soi = sibling_orbit.sphere_of_influence();
    let min_asymptote_theta = orbit.min_asymptote_theta().unwrap() + 0.001;
    let max_asymptote_theta = orbit.max_asymptote_theta().unwrap() - 0.001;
    let max_theta = find_max(&sdf, min_asymptote_theta, max_asymptote_theta);
    let max = sdf(max_theta);
    let data = BounderData {
        orbit,
        sibling_orbit,
        sibling,
        max_theta,
        min_asymptote_theta,
        max_asymptote_theta,
        soi,
    };

    if max.is_sign_negative() && max.abs() > soi {
        BounderData::no_encounters()
    } else if max.is_sign_positive() && max.abs() > soi {
        data.two_bounds(sdf)
    } else {
        data.one_bound(sdf)
    }
}