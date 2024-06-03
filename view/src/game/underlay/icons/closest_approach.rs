use eframe::egui::PointerState;
use nalgebra_glm::DVec2;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::{util::should_render_at_time, Scene};

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
        if let Some(entity) = view.selected.entity(model) {
            if let Some(vessel_component) = model.try_vessel_component(entity) {
                if let Some(target) = vessel_component.target() {
                    let (approach_1, approach_2) = model.find_next_two_closest_approaches(entity, target);
                    
                    if let Some(time) = approach_1 {
                        if should_render_at_time(view, model, entity, time) {
                            let icon = Self { entity, time, approach_number: 1 };
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                            let icon = Self { entity: target, time, approach_number: 1 };
                            icons.push(Box::new(icon) as Box<dyn Icon>);
                        }
                    }

                    if let Some(time) = approach_2 {
                        if should_render_at_time(view, model, entity, time) {
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
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        "closest-approach-".to_string() + self.approach_number.to_string().as_str()
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

    fn position(&self, _view: &Scene, model: &Model) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Closest approach position");
        // Multiply by 0.8 to offset downwards slightly
        let orbit = model.orbit_at_time(self.entity, self.time);
        model.absolute_position(orbit.parent()) + orbit.point_at_time(self.time).position()// + offset
    }

    fn facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, _view: &Scene, _model: &Model) -> bool {
        false
    }

    fn on_mouse_over(&self, _view: &mut Scene, _model: &Model, _pointer: &PointerState) {}

    fn selectable(&self) -> bool {
        false
    }
}