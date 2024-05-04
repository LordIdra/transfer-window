use eframe::egui::PointerState;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{path_component::{orbit::Orbit, segment::Segment}, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::Scene;

use super::Icon;

fn compute_time_of_next_periapsis(_model: &Model, orbit: &Orbit) -> Option<f64> {
    let periapsis_time = if orbit.is_ellipse() {
        let mut periapsis_time = orbit.first_periapsis_time();
        while periapsis_time <= orbit.current_point().time() {
            periapsis_time += orbit.period().unwrap();
        }
        periapsis_time
    } else {
        orbit.first_periapsis_time()
    };

    if periapsis_time > orbit.current_point().time() && periapsis_time < orbit.end_point().time() {
        Some(periapsis_time)
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Periapsis {
    entity: Entity,
    time: f64,
}

impl Periapsis {
    pub fn generate(model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in model.entities(vec![ComponentType::PathComponent]) {
            for segment in model.path_component(entity).future_segments().iter().flatten() {
                if let Segment::Orbit(orbit) = segment {
                    if let Some(time) = compute_time_of_next_periapsis(model, orbit) {
                        let icon = Self { entity, time };
                        icons.push(Box::new(icon) as Box<dyn Icon>);
                    }
                }
            }
        }
        icons
    }
}

impl Icon for Periapsis {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        "periapsis".to_string()
    }

    fn alpha(&self, _view: &Scene, _model: &Model, _is_selected: bool, _is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            0.2
        } else {
            1.0
        }
    }

    fn radius(&self, _view: &Scene, _model: &Model) -> f64 {
        10.0
    }

    fn priorities(&self, _view: &Scene, _model: &Model) -> [u64; 4] {
        [
            0,
            0,
            0,
            0,
        ]
    }

    fn position(&self, view: &Scene, model: &Model) -> DVec2 {
        let orbit = model.path_component(self.entity).future_segment_starting_at_time(self.time).as_orbit();
        let offset = vec2(0.0, self.radius(view, model) / view.camera.zoom());
        model.absolute_position(orbit.parent()) + orbit.position_from_theta(orbit.theta_from_time(self.time)) + offset
    }

    fn facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, _view: &Scene, _model: &Model) -> bool {
        false
    }

    fn on_mouse_over(&self, _view: &mut Scene, _model: &Model, _pointer: &PointerState) {}
}