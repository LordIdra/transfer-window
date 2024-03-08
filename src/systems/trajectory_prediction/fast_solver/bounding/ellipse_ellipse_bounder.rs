use std::f64::consts::PI;

use crate::{components::trajectory_component::orbit::Orbit, storage::entity_allocator::Entity, systems::trajectory_prediction::{fast_solver::bounding::util::{angular_distance, make_range_containing}, numerical_methods::{bisection::bisection, newton_raphson::newton_raphson}}, util::normalize_angle};

use super::{sdf::{find_min_max_signed_distance, make_sdf}, window::Window};

fn find_intersections(f: &impl Fn(f64) -> f64, min_theta: f64, max_theta: f64) -> (f64, f64) {
    let theta_1 = normalize_angle(newton_raphson(&f, bisection(&f, min_theta, max_theta), 1.0e-6).expect("Newton-Raphson failed to converge"));
    // the other angle is in the 'opposite' range
    // we can find this by subtracting 2pi from the highest theta
    let (new_min_theta, new_max_theta) = if min_theta > max_theta {
        (min_theta - 2.0 * PI, max_theta)
    } else {
        (min_theta, max_theta - 2.0 * PI)
    };
    let theta_2 = normalize_angle(newton_raphson(&f, bisection(&f, new_min_theta, new_max_theta), 1.0e-6).expect("Newton-Raphson failed to converge"));
    (theta_1, theta_2)
}

fn angle_window_to_time_window(orbit: &Orbit, window: (f64, f64)) -> (f64, f64) {
    (orbit.get_first_periapsis_time() + orbit.get_time_since_first_periapsis(window.0), orbit.get_first_periapsis_time() + orbit.get_time_since_first_periapsis(window.1))
}

struct BounderData<'a> {
    orbit: &'a Orbit, 
    sibling_orbit: &'a Orbit, 
    sibling: Entity, 
    min_theta: f64, 
    max_theta: f64, 
    soi: f64,
    start_time: f64,
    end_time: f64
}

impl<'a> BounderData<'a> {
    fn no_bounds(self) -> Vec<Window<'a>> {
        let bound = (self.start_time, self.end_time);
        let window = Window::new(self.orbit, self.sibling_orbit, self.sibling, false, bound);
        vec![window]
    }

    fn no_encounters(self) -> Vec<Window<'a>> {
        vec![]
    }

    fn one_bound(self, sdf: impl Fn(f64) -> f64) -> Vec<Window<'a>> {
        let f = |theta: f64| (sdf)(theta) - self.soi;
        let intersections = find_intersections(&f, self.min_theta, self.max_theta);
        let angle_bound = make_range_containing(intersections.0, intersections.1, self.min_theta);
        let bound = angle_window_to_time_window(&self.orbit, angle_bound);
        let window = Window::new(self.orbit, self.sibling_orbit, self.sibling, true, bound);
        vec![window]
    }

    fn two_bounds(self, sdf: impl Fn(f64) -> f64) -> Vec<Window<'a>> {
        let f_inner = |theta: f64| (sdf)(theta) - self.soi;
        let f_outer = |theta: f64| (sdf)(theta) + self.soi;
        let inner_intersections = find_intersections(&f_inner, self.min_theta, self.max_theta);
        let outer_intersections = find_intersections(&f_outer, self.min_theta, self.max_theta);
        let zero_intersections = find_intersections(&sdf, self.min_theta, self.max_theta);

        // We have 4 points, and know where the orbits intersect
        // Now we need to create two windows that cover exactly one intersection
        let from = inner_intersections.0;
        let mut possible_tos = vec![inner_intersections.1, outer_intersections.0, outer_intersections.1];
        let mut to_index = 0;
        let mut min_distance = f64::MAX;
        for i in 0..possible_tos.len() {
            let window = make_range_containing(from, possible_tos[i], zero_intersections.0);
            let distance = angular_distance(window.0, window.1);
            if distance < min_distance {
                min_distance = distance;
                to_index = i
            }
        }
        let to = possible_tos.remove(to_index);
        let angle_bound_1 = make_range_containing(from, to, zero_intersections.0);
        let angle_bound_2 = make_range_containing(possible_tos[0], possible_tos[1], zero_intersections.1);

        let bound_1 = angle_window_to_time_window(&self.orbit, angle_bound_1);
        let bound_2 = angle_window_to_time_window(&self.orbit, angle_bound_2);

        let window_1 = Window::new(self.orbit, self.sibling_orbit, self.sibling, true, bound_1);
        let window_2 = Window::new(self.orbit, self.sibling_orbit, self.sibling, true, bound_2);

        vec![window_1, window_2]
    }
}

// Bounds returned by this function assume the orbit is clockwise
pub fn get_bound<'a>(orbit: &'a Orbit, sibling_orbit: &'a Orbit, sibling: Entity, start_time: f64, end_time: f64) -> Vec<Window<'a>> {
    let argument_of_apoapsis = orbit.get_argument_of_periapsis() + PI;
    let sdf = make_sdf(orbit, sibling_orbit);
    let (min_theta, max_theta) = find_min_max_signed_distance(&sdf, argument_of_apoapsis);
    let (min, max) = (sdf(min_theta), sdf(max_theta));
    let soi = sibling_orbit.get_sphere_of_influence();
    let data = BounderData {
        orbit,
        sibling_orbit,
        sibling,
        min_theta,
        max_theta,
        soi,
        start_time,
        end_time
    };

    println!("{:.e} {:.e} {:.e}", min, max, soi);

    if min.abs() < soi && max.abs() < soi {
        data.no_bounds()
    } else if (max.is_sign_positive() && min.is_sign_positive() && min.abs() > soi) || (max.is_sign_negative() && min.is_sign_negative() && max.abs() > soi) {
        data.no_encounters()
    } else if (max.is_sign_positive() && min.is_sign_positive() && min.abs() < soi) 
           || (max.is_sign_positive() && min.is_sign_negative() && min.abs() < soi) 
           || (max.is_sign_positive() && min.is_sign_negative() && max.abs() < soi)
           || (max.is_sign_negative() && min.is_sign_negative() && max.abs() < soi) { 
        data.one_bound(sdf)
    } else {
        data.two_bounds(sdf)
    }
}
