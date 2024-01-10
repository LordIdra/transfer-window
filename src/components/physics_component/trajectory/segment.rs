use nalgebra_glm::DVec2;

use super::{burn::Burn, orbit::Orbit};

pub enum Segment {
    Orbit(Orbit),
    Burn(Burn),
}

impl Segment {
    pub fn get_start_time(&self) -> f64 {
        match self {
            Segment::Orbit(orbit) => orbit.get_start_point().get_time(),
            Segment::Burn(burn) => burn.get_start_point().get_time(),
        }
    }

    pub fn get_current_position(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.get_current_point().get_position(),
            Segment::Burn(burn) => burn.get_current_point().get_position(),
        }
    }

    pub fn get_current_velocity(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.get_current_point().get_velocity(),
            Segment::Burn(burn) => burn.get_current_point().get_velocity(),
        }
    }

    pub fn get_end_time(&self) -> f64 {
        match self {
            Segment::Orbit(orbit) => orbit.get_end_point().get_time(),
            Segment::Burn(burn) => burn.get_end_point().get_time(),
        }
    }

    pub fn get_end_position(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.get_end_point().get_position(),
            Segment::Burn(burn) => burn.get_end_point().get_position(),
        }
    }

    pub fn get_end_velocity(&self) -> DVec2 {
        match self {
            Segment::Orbit(orbit) => orbit.get_end_point().get_velocity(),
            Segment::Burn(burn) => burn.get_end_point().get_velocity(),
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            Segment::Orbit(orbit) => orbit.is_finished(),
            Segment::Burn(burn) => burn.is_finished(),
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        match self {
            Segment::Orbit(orbit) => orbit.update(delta_time),
            Segment::Burn(burn) => burn.update(delta_time),
        }
    }
}