use std::mem::swap;

use log::error;
use nalgebra_glm::DVec2;
use transfer_window_common::numerical_methods::itp::itp;

use crate::{components::{path_component::orbit::Orbit, vessel_component::Faction, ComponentType}, storage::entity_allocator::Entity, util::make_closest_point_on_ellipse_orbit_function, Model};

/// Returns the closest point to `point` on the given orbit if it is less than `radius` away from the orbit
/// Returns none if the closest distance to `point` is further than the radius
/// `point` is assumed to be relative to the parent of the orbit
fn find_closest_point_on_orbit(orbit: &Orbit, point: DVec2, max_distance: f64) -> Option<DVec2> {
    if orbit.is_ellipse() {
        let position = make_closest_point_on_ellipse_orbit_function(orbit)(point);
        if (position - point).magnitude() < max_distance {
            return Some(position);
        }
        return None;
    }

    let distance = |time: f64| (orbit.position_from_theta(orbit.theta_from_time(time)) - point).magnitude();
    let distance_prime = |time: f64| (distance(time + 1.0e-2) - distance(time)) / 1.0e-2;
    
    let min_theta = orbit.min_asymptote_theta().unwrap() + 1.0e-2;
    let max_theta = orbit.max_asymptote_theta().unwrap() - 1.0e-2;

    let mut min_time = orbit.first_periapsis_time() + orbit.time_since_first_periapsis(min_theta);
    let mut max_time = orbit.first_periapsis_time() + orbit.time_since_first_periapsis(max_theta);

    let (min, max) = (distance_prime(min_time), distance_prime(max_time));
    if min.is_sign_positive() && max.is_sign_positive() || min.is_sign_negative() && max.is_sign_negative() {
        return None;
    }
    
    if min.is_sign_positive() && max.is_sign_negative() {
        swap(&mut min_time, &mut max_time);
    }

    let time = itp(&distance_prime, min_time, max_time);
    if let Err(err) = time {
        error!("Error while computing closest point: {}", err);
        return None;
    }

    let position = orbit.position_from_theta(orbit.theta_from_time(time.unwrap()));
    if (position - point).magnitude() < max_distance {
        return Some(position);
    }
    None
}

fn process_orbit(model: &Model, orbit: &Orbit, entity: Entity, point: DVec2, max_distance: f64, closest_distance: &mut f64, closest_point: &mut Option<(Entity, f64)>) {
    let parent_position = model.absolute_position(orbit.parent());
    let point = point - parent_position;
    let Some(closest_position) = find_closest_point_on_orbit(orbit, point, max_distance) else {
        return;
    };

    let distance = (closest_position - point).magnitude();
    let theta = f64::atan2(closest_position.y, closest_position.x);
    let time = orbit.first_periapsis_time() + orbit.time_since_first_periapsis(theta);
    if distance > *closest_distance {
        return;
    }

    if time > orbit.current_point().time() && time < orbit.end_point().time() {
        *closest_point = Some((entity, time));
        *closest_distance = distance;
        return;
    }

    if let Some(period) = orbit.period() {
        // If the orbit has a period, we might have calculated an invalid time that's one period behind a valid time
        let time = time + period;
        if time > orbit.current_point().time() && time < orbit.end_point().time() {
            *closest_point = Some((entity, time));
            *closest_distance = distance;
            return;
        }

        // ffs
        let time = time + period;
        if time > orbit.current_point().time() && time < orbit.end_point().time() {
            *closest_point = Some((entity, time));
            *closest_distance = distance;
        }
    }
}

impl Model {
    /// Returns the entity and time of the closest point on ANY vessel segment provided the closest
    /// distance from the point to a segment is less than `max_distance.`
    /// Short circuits; if there are multiple points, the first one found is returned
    /// Also uses the perceived trajectories, not real ones (ie faction dependan)
    pub fn closest_point_on_any_trajectory(&self, point: DVec2, max_distance: f64, observer: Option<Faction>) -> Option<(Entity, f64)> {
        let mut closest_point = None;
        let mut closest_distance = f64::MAX;
        for entity in self.entities(vec![ComponentType::PathComponent, ComponentType::VesselComponent]) {
            for orbit in &self.future_orbits(entity, observer) {
                process_orbit(self, orbit, entity, point, max_distance, &mut closest_distance, &mut closest_point);
            }
        }
        closest_point
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use nalgebra_glm::vec2;

    use crate::{api::closest_point::find_closest_point_on_orbit, components::path_component::orbit::Orbit, storage::entity_allocator::Entity};

    #[test]
    fn test_find_closest_point_on_orbit_ellipse() {
        // Earth orbiting sun
        let orbit = Orbit::new(Entity::mock(), 5.9722e24, 1_988_500e24, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0);

        let c = find_closest_point_on_orbit(&orbit, vec2(1.5e11, 0.0), 1.0e10).unwrap();
        assert!(f64::atan2(c.y, c.x).abs() < 1.0e-2);
        let c = find_closest_point_on_orbit(&orbit, vec2(-1.5e11, -1.0e7), 1.0e10).unwrap();
        assert!((f64::atan2(c.y, c.x) + PI).abs() < 1.0e-2);
        let c = find_closest_point_on_orbit(&orbit, vec2(1.5e11, 0.0), 1.0e7);
        assert!(c.is_none());
        let c = find_closest_point_on_orbit(&orbit, vec2(-1.5e11, 0.0), 1.0e7);
        assert!(c.is_none());
    }

    #[test]
    fn test_find_closest_point_on_orbit_hyperbola() {
        // Hyperbolic moon
        let orbit = Orbit::new(Entity::mock(), 0.07346e24, 5.9722e24, vec2(0.3633e9, 0.0), vec2(0.0, 2.082e3), 0.0);

        let c = find_closest_point_on_orbit(&orbit, vec2(0.36e9, 0.0), 1.0e7).unwrap();
        assert!(f64::atan2(c.y, c.x).abs() < 1.0e-2);
        let c = find_closest_point_on_orbit(&orbit, vec2(1.5e11, 0.0), 1.0e5);
        assert!(c.is_none());
        let c = find_closest_point_on_orbit(&orbit, vec2(-1.5e11, 0.0), 1.0e5);
        assert!(c.is_none());
    }
}