use std::mem::swap;

use log::error;
use nalgebra_glm::DVec2;
use transfer_window_common::numerical_methods::itp::itp;

use crate::components::path_component::burn::Burn;
use crate::components::path_component::guidance::Guidance;
use crate::components::path_component::orbit::Orbit;
use crate::components::vessel_component::faction::Faction;
use crate::components::ComponentType;
use crate::storage::entity_allocator::Entity;
use crate::util::make_closest_point_on_ellipse_orbit_function;
use crate::Model;

/// Returns the closest point to `point` on the given orbit if it is less than
/// `radius` away from the orbit Returns none if the closest distance to `point`
/// is further than the radius `point` is assumed to be relative to the parent
/// of the orbit
fn find_closest_point_on_orbit(orbit: &Orbit, point: DVec2, max_distance: f64) -> Option<DVec2> {
    if orbit.is_ellipse() {
        let position = make_closest_point_on_ellipse_orbit_function(orbit)(point);
        if (position - point).magnitude() < max_distance {
            return Some(position);
        }
        return None;
    }

    let distance =
        |time: f64| (orbit.position_from_theta(orbit.theta_from_time(time)) - point).magnitude();
    let distance_prime = |time: f64| (distance(time + 1.0e-2) - distance(time)) / 1.0e-2;

    let min_theta = orbit.min_asymptote_theta().unwrap() + 1.0e-2;
    let max_theta = orbit.max_asymptote_theta().unwrap() - 1.0e-2;

    let mut min_time = orbit.first_periapsis_time() + orbit.time_since_first_periapsis(min_theta);
    let mut max_time = orbit.first_periapsis_time() + orbit.time_since_first_periapsis(max_theta);

    let (min, max) = (distance_prime(min_time), distance_prime(max_time));
    if min.is_sign_positive() && max.is_sign_positive()
        || min.is_sign_negative() && max.is_sign_negative()
    {
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

fn process_orbit(
    orbit: &Orbit,
    entity: Entity,
    point: DVec2,
    max_distance: f64,
    closest_distance: &mut f64,
    closest_point: &mut Option<(Entity, f64)>,
) {
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
        // If the orbit has a period, we might have calculated an invalid time that's
        // one period behind a valid time
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

fn itp_to_find_turning_point(
    initial_points: &[(f64, f64)],
    derivative: impl Fn(f64) -> f64,
    distance: impl Fn(f64) -> f64,
    max_distance: f64,
    closest_distance: &mut f64,
    closest_point: &mut Option<(Entity, f64)>,
    entity: Entity,
) {
    // Find any pairs of points where the distance derivative flips from negative to
    // positive or vice versa This gives us all the minima/maxima on the burn
    // When a pair is found, use the ITP solver to find the time of the
    // minimum/maximum
    let (mut previous_time, mut previous_distance_derivative) = initial_points.first().unwrap();
    for (current_time, current_distance_derivative) in initial_points.iter().skip(1) {
        let time = if previous_distance_derivative.is_sign_negative()
            && current_distance_derivative.is_sign_positive()
        {
            Some(itp(&derivative, previous_time, *current_time))
        } else if previous_distance_derivative.is_sign_positive()
            && current_distance_derivative.is_sign_negative()
        {
            Some(itp(&derivative, *current_time, previous_time))
        } else {
            None
        };
        if let Some(time) = time {
            match time {
                Ok(time) => {
                    let distance = distance(time);
                    if distance < max_distance && distance < *closest_distance {
                        *closest_distance = distance;
                        *closest_point = Some((entity, time));
                    }
                }
                Err(err) => error!("Error while computing closest point: {}", err),
            }
        }
        previous_time = *current_time;
        previous_distance_derivative = *current_distance_derivative;
    }
}

fn process_burn(
    burn: &Burn,
    entity: Entity,
    point: DVec2,
    max_distance: f64,
    closest_distance: &mut f64,
    closest_point: &mut Option<(Entity, f64)>,
) {
    if burn.duration() < 0.03 {
        return;
    }

    let distance = |time: f64| (burn.point_at_time(time).position() - point).magnitude();
    let derivative = |time: f64| (distance(time + 0.0001) - distance(time)) / 0.0001;

    // Sample 10 evenly distributed points from burn start to end, finding their
    // derivatives
    let mut initial_points = vec![];
    for i in 0..=10 {
        let mut time = burn.start_point().time() + (i as f64 / 10.0) * burn.duration();
        if i == 10 {
            // Derivative calculations would otherwise go beyond end of guidance
            time -= 0.0001;
        }
        initial_points.push((time, derivative(time)));
    }

    itp_to_find_turning_point(
        &initial_points,
        derivative,
        distance,
        max_distance,
        closest_distance,
        closest_point,
        entity,
    );
}

fn process_guidance(
    guidance: &Guidance,
    entity: Entity,
    point: DVec2,
    max_distance: f64,
    closest_distance: &mut f64,
    closest_point: &mut Option<(Entity, f64)>,
) {
    if guidance.duration() < 0.03 {
        return;
    }

    let distance = |time: f64| (guidance.point_at_time(time).position() - point).magnitude();
    let derivative = |time: f64| (distance(time + 0.0001) - distance(time)) / 0.0001;

    // Sample 10 evenly distributed points from burn start to end, finding their
    // derivatives
    let mut initial_points = vec![];
    for i in 0..=10 {
        let mut time = guidance.start_point().time() + (i as f64 / 10.0) * guidance.duration();
        if i == 10 {
            // Derivative calculations would otherwise go beyond end of guidance
            time -= 0.0001;
        }
        initial_points.push((time, derivative(time)));
    }

    itp_to_find_turning_point(
        &initial_points,
        derivative,
        distance,
        max_distance,
        closest_distance,
        closest_point,
        entity,
    );
}

impl Model {
    /// Returns the entity and time of the closest point on ANY vessel orbit
    /// provided the closest distance from the point to a segment is less
    /// than `max_distance.`
    pub fn closest_orbit_point(
        &self,
        point: DVec2,
        max_distance: f64,
        observer: Option<Faction>,
    ) -> Option<(Entity, f64)> {
        let mut closest_point = None;
        let mut closest_distance = f64::MAX;
        for entity in self.entities(vec![
            ComponentType::PathComponent,
            ComponentType::VesselComponent,
        ]) {
            let point = point - self.absolute_position(self.parent(entity).unwrap());
            for orbit in self.future_orbits(entity, observer) {
                process_orbit(
                    orbit,
                    entity,
                    point,
                    max_distance,
                    &mut closest_distance,
                    &mut closest_point,
                );
            }
        }
        closest_point
    }

    /// Returns the entity and time of the closest point on ANY vessel burn
    /// provided the closest distance from the point to a segment is less
    /// than `max_distance.`
    pub fn closest_burn_point(
        &self,
        point: DVec2,
        max_distance: f64,
        observer: Option<Faction>,
    ) -> Option<(Entity, f64)> {
        let mut closest_point = None;
        let mut closest_distance = f64::MAX;
        for entity in self.entities(vec![
            ComponentType::PathComponent,
            ComponentType::VesselComponent,
        ]) {
            let point = point - self.absolute_position(self.parent(entity).unwrap());
            for burn in self.future_burns(entity, observer) {
                process_burn(
                    burn,
                    entity,
                    point,
                    max_distance,
                    &mut closest_distance,
                    &mut closest_point,
                );
            }
        }
        closest_point
    }

    /// Returns the entity and time of the closest point on ANY vessel burn
    /// provided the closest distance from the point to a segment is less
    /// than `max_distance.`
    pub fn closest_guidance_point(
        &self,
        point: DVec2,
        max_distance: f64,
        observer: Option<Faction>,
    ) -> Option<(Entity, f64)> {
        let mut closest_point = None;
        let mut closest_distance = f64::MAX;
        for entity in self.entities(vec![
            ComponentType::PathComponent,
            ComponentType::VesselComponent,
        ]) {
            let point = point - self.absolute_position(self.parent(entity).unwrap());
            for guidance in self.future_guidances(entity, observer) {
                process_guidance(
                    guidance,
                    entity,
                    point,
                    max_distance,
                    &mut closest_distance,
                    &mut closest_point,
                );
            }
        }
        closest_point
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use nalgebra_glm::vec2;

    use crate::api::closest_point::find_closest_point_on_orbit;
    use crate::components::path_component::orbit::Orbit;
    use crate::storage::entity_allocator::Entity;

    #[test]
    fn test_find_closest_point_on_orbit_ellipse() {
        // Earth orbiting sun
        let orbit = Orbit::new(
            Entity::mock(),
            5.9722e24,
            1_988_500e24,
            vec2(147.095e9, 0.0),
            vec2(0.0, 30.29e3),
            0.0,
        );

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
        let orbit = Orbit::new(
            Entity::mock(),
            0.07346e24,
            5.9722e24,
            vec2(0.3633e9, 0.0),
            vec2(0.0, 2.082e3),
            0.0,
        );

        let c = find_closest_point_on_orbit(&orbit, vec2(0.36e9, 0.0), 1.0e7).unwrap();
        assert!(f64::atan2(c.y, c.x).abs() < 1.0e-2);
        let c = find_closest_point_on_orbit(&orbit, vec2(1.5e11, 0.0), 1.0e5);
        assert!(c.is_none());
        let c = find_closest_point_on_orbit(&orbit, vec2(-1.5e11, 0.0), 1.0e5);
        assert!(c.is_none());
    }
}
