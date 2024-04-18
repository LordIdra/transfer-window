use eframe::egui::PointerState;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::Scene;

use super::Icon;

#[derive(Debug)]
pub struct ClosestApproach {
    entity: Entity,
    time: f64,
    approach_number: usize,
}

impl ClosestApproach {
    pub fn generate(view: &Scene, model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        if let Some(entity) = view.camera.get_focus() {
            if let Some(vessel_component) = model.try_get_vessel_component(entity) {
                if let Some(target) = vessel_component.get_target() {
                    if let Some(time) = model.find_next_closest_approach(entity, target, model.get_time()) {
                        // 1st closest approach
                        let icon = Self { entity, time, approach_number: 1 };
                        icons.push(Box::new(icon) as Box<dyn Icon>);
                        let icon = Self { entity: target, time, approach_number: 1 };
                        icons.push(Box::new(icon) as Box<dyn Icon>);

                        // 2nd closest approach
                        // Add 1.0 to make sure we don't find the same approach by accident
                        if let Some(time) = model.find_next_closest_approach(entity, target, time + 1.0) {
                            let icon = Self { entity, time, approach_number: 2 };
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                            let icon = Self { entity: target, time, approach_number: 2 };
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                        }
                    }
                }
            }
        }
        icons
    }
}

impl Icon for ClosestApproach {
    fn get_texture(&self, _view: &Scene, _model: &Model) -> String {
        "closest-approach-".to_string() + self.approach_number.to_string().as_str()
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
        let offset = vec2(0.0, self.get_radius(view, model) / view.camera.get_zoom());
        let segment = model.get_trajectory_component(self.entity).get_first_segment_at_time(self.time);
        model.get_absolute_position(segment.get_parent()) + segment.get_position_at_time(self.time) + offset
    }

    fn get_facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, _view: &Scene, _model: &Model) -> bool {
        false
    }

    fn on_mouse_over(&self, _view: &mut Scene, _model: &Model, _pointer: &PointerState) {}
}