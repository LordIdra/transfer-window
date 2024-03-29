use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use self::{ellipse::Ellipse, hyperbola::Hyperbola};

use super::{orbit_direction::OrbitDirection, scary_math::{eccentricity, semi_major_axis, GRAVITATIONAL_CONSTANT}};

mod ellipse;
mod hyperbola;

pub enum ConicType {
    Ellipse,
    Hyperbola,
}

/// Describes all the static parmeters of an orbit, but says nothing about the current model of the object in the orbit
/// We use an enum instead of dynamic dispatch here because we cannot serialize trait objects
#[derive(Debug, Serialize, Deserialize)]
pub enum Conic {
    Ellipse(Ellipse),
    Hyperbola(Hyperbola),
}

impl Conic {
    pub fn new(parent_mass: f64, position: DVec2, velocity: DVec2) -> Self {
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * parent_mass;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::new(position, velocity);
        if eccentricity <= 1.0 {
            Conic::Ellipse(Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction))
        } else {
            Conic::Hyperbola(Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction))
        }
    }

    pub fn get_theta_from_time_since_periapsis(&self, time_since_periapsis: f64) -> f64 {
        match self {
            Conic::Ellipse(ellipse) => ellipse.get_theta_from_time_since_periapsis(time_since_periapsis),
            Conic::Hyperbola(hyperbola) => hyperbola.get_theta_from_time_since_periapsis(time_since_periapsis),
        }
    }

    pub fn get_time_since_last_periapsis(&self, theta: f64) -> f64 {
        match self {
            Conic::Ellipse(ellipse) => ellipse.get_time_since_last_periapsis(theta),
            Conic::Hyperbola(hyperbola) => hyperbola.get_time_since_last_periapsis(theta),
        }
    }

    pub fn get_position(&self, theta: f64) -> DVec2 {
        match self {
            Conic::Ellipse(ellipse) => ellipse.get_position(theta),
            Conic::Hyperbola(hyperbola) => hyperbola.get_position(theta),
        }
    }

    pub fn get_velocity(&self, position: DVec2, theta: f64) -> DVec2 {
        match self {
            Conic::Ellipse(ellipse) => ellipse.get_velocity(position, theta),
            Conic::Hyperbola(hyperbola) => hyperbola.get_velocity(position, theta),
        }
    }

    pub fn get_type(&self) -> ConicType {
        match self {
            Conic::Ellipse(_) => ConicType::Ellipse,
            Conic::Hyperbola(_) => ConicType::Hyperbola,
        }
    }

    pub fn get_direction(&self) -> OrbitDirection {
        match self {
            Conic::Ellipse(ellipse) => ellipse.get_direction(),
            Conic::Hyperbola(hyperbola) => hyperbola.get_direction(),
        }
    }

    pub fn get_semi_major_axis(&self) -> f64 {
        match self {
            Conic::Ellipse(ellipse) => ellipse.get_semi_major_axis(),
            Conic::Hyperbola(hyperbola) => hyperbola.get_semi_major_axis(),
        }
    }

    pub fn get_semi_minor_axis(&self) -> f64 {
        match self {
            Conic::Ellipse(ellipse) => ellipse.get_semi_minor_axis(),
            Conic::Hyperbola(hyperbola) => hyperbola.get_semi_minor_axis(),
        }
    }

    pub fn get_argument_of_periapsis(&self) -> f64 {
        match self {
            Conic::Ellipse(ellipse) => ellipse.get_argument_of_periapsis(),
            Conic::Hyperbola(hyperbola) => hyperbola.get_argument_of_periapsis(),
        }
    }

    pub fn get_min_asymptote_theta(&self) -> Option<f64> {
        match self {
            Conic::Ellipse(_) => None,
            Conic::Hyperbola(hyperbola) => Some(hyperbola.get_min_asymptote_theta()),
        }
    }

    pub fn get_max_asymptote_theta(&self) -> Option<f64> {
        match self {
            Conic::Ellipse(_) => None,
            Conic::Hyperbola(hyperbola) => Some(hyperbola.get_max_asymptote_theta()),
        }
    }

    pub fn get_eccentricity(&self) -> f64 {
        match self {
            Conic::Ellipse(ellipse) => ellipse.get_eccentricity(),
            Conic::Hyperbola(hyperbola) => hyperbola.get_eccentricity(),
        }
    }

    pub fn get_period(&self) -> Option<f64> {
        match self {
            Conic::Ellipse(ellipse) => Some(ellipse.get_period()),
            Conic::Hyperbola(_) => None,
        }
    }

    pub fn get_orbits(&self, time: f64) -> i32 {
        match self {
            Conic::Ellipse(ellipse) => ellipse.get_orbits(time),
            Conic::Hyperbola(_) => 0
        }
    }
}