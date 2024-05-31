use eframe::egui::PointerState;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{path_component::orbit::Orbit, ComponentType}, storage::entity_allocator::Entity, Model};

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
    position: DVec2,
}

impl Periapsis {
    fn new(_view: &Scene, model: &Model, entity: Entity, time: f64) -> Self {
        let orbit = model.orbit_at_time(entity, time);
        let position = model.absolute_position(orbit.parent()) + orbit.point_at_time(time).position();
        Self { position }
    }

    pub fn generate(view: &Scene, model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in model.entities(vec![ComponentType::PathComponent]) {
            for orbit in model.path_component(entity).future_orbits() {
                if let Some(time) = compute_time_of_next_periapsis(model, orbit) {
                    let icon = Self::new(view, model, entity, time);
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }
        
        for entity in model.entities(vec![ComponentType::OrbitableComponent]) {
            if let Some(orbit) = model.orbitable_component(entity).orbit() {
                if let Some(time) = compute_time_of_next_periapsis(model, orbit) {
                    let icon = Self::new(view, model, entity, time);
                    icons.push(Box::new(icon) as Box<dyn Icon>);
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
            0.4
        } else {
            1.0
        }
    }

    fn radius(&self, _view: &Scene, _model: &Model) -> f64 {
        8.0
    }

    fn priorities(&self, _view: &Scene, _model: &Model) -> [u64; 4] {
        [
            0,
            0,
            0,
            0,
        ]
    }

    fn position(&self, _view: &Scene, _model: &Model) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Periapsis position");
        self.position
    }

    fn facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, _view: &Scene, _model: &Model) -> bool {
        false
    }

    fn on_mouse_over(&self, _view: &mut Scene, _model: &Model, _pointer: &PointerState) {}
}