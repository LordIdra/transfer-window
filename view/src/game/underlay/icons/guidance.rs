use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::ComponentType, storage::entity_allocator::Entity, Model};

use crate::game::{selected::Selected, util::should_render_at_time, Scene};

use super::Icon;

#[derive(Debug)]
pub struct Guidance {
    entity: Entity,
    time: f64,
}

impl Guidance {
    pub fn generate(view: &Scene, model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.entities_should_render(model, vec![ComponentType::VesselComponent]) {
            for event in model.vessel_component(entity).timeline().events() {
                if event.is_enable_guidance() && should_render_at_time(view, model, entity, event.time()) {
                    let icon = Self { entity, time: event.time() };
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }
        icons
    }
}

impl Icon for Guidance {
    fn texture(&self, _view: &Scene, _model: &Model) -> String {
        "guidance".to_string()
    }

    fn alpha(&self, _view: &Scene, _model: &Model, is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            return 0.4;
        }
        if is_selected {
            return 1.0;
        }
        if is_hovered {
            return 0.8
        }
        0.6
    }

    fn radius(&self, _view: &Scene, _model: &Model) -> f64 {
        18.0
    }

    fn priorities(&self, view: &Scene, model: &Model) -> [u64; 4] {
        [
            u64::from(self.is_selected(view, model)),
            0,
            0,
            model.mass(self.entity) as u64
        ]
    }

    fn position(&self, _view: &Scene, model: &Model) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Guidance position");
        let guidance = model.path_component(self.entity)
            .future_segment_starting_at_time(self.time)
            .expect("Selected guidance does not exist")
            .as_guidance()
            .expect("Selected guidance is not guidance segment");
        model.absolute_position(guidance.parent()) + guidance.start_point().position()
    }

    fn facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        if let Selected::EnableGuidance { entity, time } = &view.selected {
            *entity == self.entity && *time == self.time
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &mut Scene, _model: &Model, pointer: &PointerState) {
        if !pointer.primary_clicked() {
            return;
        }

        trace!("Guidance icon clicked; switching to Selected");
        view.selected = Selected::EnableGuidance { entity: self.entity, time: self.time }
    }

    fn selectable(&self) -> bool {
        true
    }
}