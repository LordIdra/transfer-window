use eframe::egui::PointerState;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{path_component::{orbit::Orbit, segment::Segment}, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::Scene;

use super::Icon;

fn get_time_of_next_apoapsis(_model: &Model, orbit: &Orbit) -> Option<f64> {
    if !orbit.is_ellipse() {
        return None;
    }

    let mut apoapsis_time = orbit.first_periapsis_time() - orbit.period().unwrap() / 2.0;
    while apoapsis_time <= orbit.current_point().time() {
        apoapsis_time += orbit.period().unwrap();
    }
    if apoapsis_time > orbit.current_point().time() && apoapsis_time < orbit.end_point().time() {
        Some(apoapsis_time)
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Apoapsis {
    entity: Entity,
    time: f64,
}

impl Apoapsis {
    pub fn generate(model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in model.get_entities(vec![ComponentType::PathComponent]) {
            for segment in model.get_path_component(entity).get_segments().iter().flatten() {
                if let Segment::Orbit(orbit) = segment {
                    if let Some(time) = get_time_of_next_apoapsis(model, orbit) {
                        let icon = Self { entity, time };
                        icons.push(Box::new(icon) as Box<dyn Icon>);
                    }
                }
            }
        }
        icons
    }
}

impl Icon for Apoapsis {
    fn get_texture(&self, _view: &Scene, _model: &Model) -> String {
        "apoapsis".to_string()
    }

    fn get_alpha(&self, _view: &Scene, _model: &Model, _is_selected: bool, _is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            0.2
        } else {
            1.0
        }
    }

    fn get_radius(&self, _view: &Scene, _model: &Model) -> f64 {
        10.0
    }

    fn get_priorities(&self, _view: &Scene, _model: &Model) -> [u64; 4] {
        [
            0,
            0,
            0,
            0,
        ]
    }

    fn get_position(&self, view: &Scene, model: &Model) -> DVec2 {
        let orbit = model.get_path_component(self.entity).get_last_segment_at_time(self.time).as_orbit();
        let offset = vec2(0.0, self.get_radius(view, model) / view.camera.get_zoom());
        model.get_absolute_position(orbit.parent()) + orbit.position_from_theta(orbit.theta_from_time(self.time)) + offset
    }

    fn get_facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, _view: &Scene, _model: &Model) -> bool {
        false
    }

    fn on_mouse_over(&self, _view: &mut Scene, _model: &Model, _pointer: &PointerState) {}
}