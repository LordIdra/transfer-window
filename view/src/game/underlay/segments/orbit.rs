use std::f64::consts::PI;

use eframe::emath::normalized_angle;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::components::trajectory_component::orbit::Orbit;

const INITIAL_POINT_COUNT: usize = 100;
const INITIAL_THETA_INCREMENT: f64 = 1.0e-8;
const TESSELLATION_THRESHOLD: f64 = 1.0e-3;

fn find_initial_points_orbit(orbit: &Orbit, absolute_parent_position: DVec2) -> Vec<DVec2> {
    let start_angle = normalized_angle(orbit.get_start_point().get_theta() as f32);
    let angle_to_rotate_through = orbit.get_remaining_angle();

    // First and last points are be equal to make sure we draw lines through the whole segment
    let mut initial_points = vec![];
    for i in 0..=INITIAL_POINT_COUNT {
        let theta = start_angle as f64 + (i as f64 / INITIAL_POINT_COUNT as f64) * angle_to_rotate_through;
        initial_points.push(absolute_parent_position + orbit.get_position_from_theta(theta));
    }

    initial_points
}

/// Splits the orbit into sections contained within the camera radius
/// Each section contains adjacent points which are contained within the view circle or adjacent to a point inside it
/// Also computes the closest initial point to the camera centre
fn find_initial_sections(initial_points: &Vec<DVec2>, camera_centre: DVec2, radius: f64, is_ellipse: bool) -> (Vec<Vec<DVec2>>, DVec2) {
    let mut segment_sections: Vec<Vec<DVec2>> = vec![];
    let mut is_in_circle = false;
    let mut min_distance = f64::MAX;
    let mut min_point = None;
    for i in 0..initial_points.len() {
        let distance = (camera_centre - initial_points[i]).magnitude();

        if distance < min_distance {
            min_distance = distance;
            min_point = Some(initial_points[i]);
        }

        if is_in_circle {
            // We first push the point, then check if we've exited the circle
            // This means that one point will be pushed after exiting the circle (because it is adjacent to a point inside)
            segment_sections.last_mut().unwrap().push(initial_points[i]);
            if distance > radius {
                is_in_circle = false;
            }
        } else {
            if distance < radius {
                // Start a new section
                segment_sections.push(vec![]);

                // Push i-1 BEFORE the current point to make sure we retain correct order
                if i != 0 {
                    segment_sections.last_mut().unwrap().push(initial_points[i - 1]);
                }

                segment_sections.last_mut().unwrap().push(initial_points[i]);
                is_in_circle = true;
            }
        }
    }

    // Special case: orbit is an ellipse and both the first and last points are on screen
    // In this case we should merge the first and last sections, otherwise problems can occurr downstream
    if is_ellipse {
        let first_in_circle = (camera_centre - initial_points.first().unwrap()).magnitude() < radius;
        let last_in_circle = (camera_centre - initial_points.last().unwrap()).magnitude() < radius;
        if first_in_circle && last_in_circle && segment_sections.len() >= 2 {
            segment_sections.first_mut().unwrap().remove(0); // Remove duplicate point
            let mut first = segment_sections.remove(0);
            segment_sections.last_mut().unwrap().append(&mut first);
        }
    }

    (segment_sections, min_point.unwrap())
}

/// Assumes the angles are within 2pi of each other
fn interpolate_angles(mut angle_1: f64, mut angle_2: f64) -> f64 {
    // This is confusing, but helps prevent edge cases where eg 6.0 and 0.1 have an 'average' of 3.05
    if (angle_1 - angle_2).abs() > PI {
        if angle_1 < angle_2 {
            angle_1 += 2.0 * PI;
        } else {
            angle_2 += 2.0 * PI;
        }
    }
    (angle_1 + angle_2) / 2.0
}

// Uses triangle heuristic as described in https://www.kerbalspaceprogram.com/news/dev-diaries-orbit-tessellation
fn tessellate(orbit: &Orbit, mut section: Vec<DVec2>, absolute_parent_position: DVec2, camera_centre: DVec2, zoom: f64) -> Vec<DVec2> {
    // Make the input relative instead of absolute
    for point in &mut section {
        *point -= absolute_parent_position;
    }

    let mut i = 2;
    while i < section.len() {
        let point1 = section[i-2];
        let point2 = section[i-1];
        let point3 = section[i];

        let point1_screen_space = (point1 + absolute_parent_position - camera_centre) * zoom;
        let point2_screen_space = (point2 + absolute_parent_position - camera_centre) * zoom;
        let point3_screen_space = (point3 + absolute_parent_position - camera_centre) * zoom;
        
        // Heron's method, see https://www.mathopenref.com/heronsformula.html
        let a = (point1_screen_space - point2_screen_space).magnitude();
        let b = (point1_screen_space - point3_screen_space).magnitude();
        let c = (point2_screen_space - point3_screen_space).magnitude();

        let p = (a + b + c) / 2.0;
        let area = f64::sqrt(p * (p - a) * (p - b) * (p - c));

        let min_distance = f64::min(point1_screen_space.magnitude_squared(), f64::min(point2_screen_space.magnitude_squared(), point3_screen_space.magnitude_squared()));
        trace!("{}", area / min_distance);

        if area / min_distance > TESSELLATION_THRESHOLD {
            let theta_1 = f64::atan2(point1.y, point1.x);
            let theta_2 = f64::atan2(point2.y, point2.x);
            let theta_3 = f64::atan2(point3.y, point3.x);

            let new_point_1 = orbit.get_position_from_theta(interpolate_angles(theta_1, theta_2));
            let new_point_2 = orbit.get_position_from_theta(interpolate_angles(theta_2, theta_3));

            section.insert(i - 1, new_point_1);
            section.insert(i + 1, new_point_2);
        } else {
            i += 1;
        }
    }

    trace!("Tessellated to {} points", section.len());

    // Make the output absolute instead of relative
    for point in &mut section {
        *point += absolute_parent_position;
    }

    section
}

/// Finds the maximum distance between any two adjacent initial points
fn find_maximum_distance(initial_points: &Vec<DVec2>) -> f64{
    let mut max = 0.0;
    for i in 1..initial_points.len() {
        let distance = (initial_points[i-1] - initial_points[i]).magnitude();
        max = f64::max(max, distance)
    }
    max
}

pub fn approximate_section(orbit: &Orbit, absolute_parent_position: DVec2, camera_centre: DVec2, radius: f64, zoom: f64) -> Vec<DVec2> {
    // If we've got this far, we may or may not have part the conic on screen
    // Either way, we don't have any points within the view radius
    // So next step: find out if the conic will intersect the view circle
    // We can assume that the closest point on our orbit is at relative_initial_min_point
    // This assumption seems to hold up incredibly well and produce ridiculously accurate results
    // We use 5.0 * radius as a safe buffer
    let relative_initial_min_point = camera_centre - absolute_parent_position;
    let min_theta = f64::atan2(relative_initial_min_point.y, relative_initial_min_point.x);
    let sdf = |theta: f64| (orbit.get_position_from_theta(theta) + absolute_parent_position - camera_centre).magnitude() - radius;
    if sdf(min_theta).is_sign_positive() {
        // The orbit is probably off-screen
        trace!("3");
        return vec![]
    }

    // (part of the) orbit is not off-screen
    // Strategy now is to move in either direction until we're outside of the view circle again
    // We'll do this by starting with a small increment and doubling, so time complexity logN
    // This will give us 3 points to start with
    trace!("4");
    let mut increment = INITIAL_THETA_INCREMENT;
    let mut theta_forward = min_theta;
    while sdf(theta_forward).is_sign_negative() {
        theta_forward += increment;
        increment *= 2.0;
    }

    let mut increment = INITIAL_THETA_INCREMENT;
    let mut theta_backward = min_theta;
    while sdf(theta_backward).is_sign_negative() {
        theta_backward -= increment;
        increment *= 2.0;
    }

    let section = vec![
        absolute_parent_position + orbit.get_position_from_theta(theta_backward),
        absolute_parent_position + orbit.get_position_from_theta(min_theta),
        absolute_parent_position + orbit.get_position_from_theta(theta_forward),
    ];

    tessellate(orbit, section, absolute_parent_position, camera_centre, zoom)
}

pub fn compute_sections(orbit: &Orbit, absolute_parent_position: DVec2, camera_centre: DVec2, zoom: f64, radius: f64) -> Vec<Vec<DVec2>> {
    // let initial_points = find_initial_points_orbit(orbit, absolute_parent_position);
    // let maximum_distance_between_adjacent_points = find_maximum_distance(&initial_points);
    // let (initial_sections, closest_point_to_camera) = find_initial_sections(&initial_points, camera_centre, radius, orbit.is_ellipse());
    // trace!("{} sections in {:?}", initial_sections.len(), initial_sections);
    // let closest_distance_to_camera = (closest_point_to_camera - camera_centre).magnitude();

    // if closest_distance_to_camera > maximum_distance_between_adjacent_points + radius {
    //     // The entire orbit is most likely off screen
    //     trace!("1");
    //     return vec![];
    // }

    // if !initial_sections.is_empty() {
    //     let mut new_sections = vec![];
    //     for initial_section in initial_sections {
    //         new_sections.push(tessellate(orbit, initial_section, absolute_parent_position, camera_centre, zoom));
    //     }
    //     return new_sections;
    // }

    // // This path is triggered when we're close to the orbit but have not computed any sections
    // // This implies we have might have exactly one section on screen, but if we do, the points
    // // we've computed don't appear on screen
    // vec![approximate_section(orbit, absolute_parent_position, camera_centre, radius, zoom)]
    let initial_points = find_initial_points_orbit(orbit, absolute_parent_position);
    vec![tessellate(orbit, initial_points, absolute_parent_position, camera_centre, zoom)]
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::game::underlay::segments::orbit::interpolate_angles;

    #[test]
    fn test_interpolate_angles() {
        assert!((interpolate_angles(0.1, 0.3) - 0.2).abs() < 1.0e-3);
        assert!((interpolate_angles(-0.1, 0.3) - 0.1).abs() < 1.0e-3);
        assert!((interpolate_angles(-0.1, 2.3) - 1.1).abs() < 1.0e-3);
        assert!((interpolate_angles(-3.0, 3.0) - PI).abs() < 1.0e-3);
    }
}