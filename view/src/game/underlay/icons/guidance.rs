use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::ComponentType, storage::entity_allocator::Entity};

use crate::game::{events::ViewEvent, selected::Selected, util::should_render_at_time, View};

use super::Icon;

#[derive(Debug)]
pub struct Guidance {
    entity: Entity,
    time: f64,
}

impl Guidance {
    pub fn generate(view: &View) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.entities_should_render(vec![ComponentType::VesselComponent]) {
            for event in view.model.vessel_component(entity).timeline().events() {
                if event.is_enable_guidance() && should_render_at_time(view, entity, event.time()) {
                    let icon = Self { entity, time: event.time() };
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }
        icons
    }
}

impl Icon for Guidance {
    fn texture(&self, _view: &View) -> String {
        "guidance".to_string()
    }

    fn alpha(&self, _view: &View, is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
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

    fn radius(&self, _view: &View) -> f64 {
        18.0
    }

    fn priorities(&self, view: &View) -> [u64; 4] {
        [
            u64::from(self.is_selected(view)),
            0,
            5,
            view.model.mass(self.entity) as u64
        ]
    }

    fn position(&self, view: &View) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Guidance position");
        let guidance = view.model.path_component(self.entity)
            .future_segment_starting_at_time(self.time)
            .expect("Selected guidance does not exist")
            .as_guidance()
            .expect("Selected guidance is not guidance segment");
        view.model.absolute_position(guidance.parent()) + guidance.start_point().position()
    }

    fn facing(&self, _view: &View) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &View) -> bool {
        if let Selected::EnableGuidance { entity, time } = &view.selected {
            *entity == self.entity && *time == self.time
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &View, pointer: &PointerState) {
        if !pointer.primary_clicked() {
            return;
        }

        trace!("Guidance icon clicked; switching to Selected");
        let selected = Selected::EnableGuidance { entity: self.entity, time: self.time };
        view.add_view_event(ViewEvent::SetSelected(selected));
    }

    fn selectable(&self) -> bool {
        true
    }
}