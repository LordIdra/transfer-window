use eframe::egui::Rgba;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{trajectory_component::segment::Segment, ComponentType}, storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::{BurnState, Selected}, Scene};

use super::Icon;

#[derive(Debug)]
pub struct Burn {
    entity: Entity,
    time: f64,
}

impl Burn {
    pub fn generate(model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in model.get_entities(vec![ComponentType::VesselComponent]) {
            for segment in model.get_trajectory_component(entity).get_segments().iter().flatten() {
                if let Segment::Burn(burn) = segment {
                    let time = burn.get_start_point().get_time();
                    let icon = Self { entity, time };
                    icons.push(Box::new(icon) as Box<dyn Icon>);
                }
            }
        }
        icons
    }
}

impl Icon for Burn {
    fn get_texture(&self) -> &str {
        "burn"
    }

    fn get_color(&self) -> eframe::egui::Rgba {
        Rgba::from_rgb(1.0, 0.7, 0.5)
    }

    fn get_radius(&self) -> f64 {
        10.0
    }

    fn get_priorities(&self, view: &Scene, model: &Model) -> [u64; 4] {
        [
            u64::from(self.is_selected(view, model)),
            0,
            0,
            (model.get_mass_component(self.entity).get_mass() / 1.0e22) as u64
        ]
    }

    fn get_position(&self, _view: &Scene, model: &Model) -> DVec2 {
        let burn = model.get_trajectory_component(self.entity).get_segment_at_time(self.time).as_burn();
        model.get_absolute_position(burn.get_parent()) + burn.get_start_point().get_position()
    }

    fn get_facing(&self, _view: &Scene, _model: &Model) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        #[allow(clippy::float_cmp)] // time and self.time should be *exactly* the same
        match &view.selected {
            Selected::None | Selected::Point { entity: _, time: _, state: _ } => false,
            Selected::Burn { entity, time, state: _ } => *entity == self.entity && *time == self.time,
        }
    }

    fn on_clicked(&self, view: &mut Scene, _model: &Model) {
        if let Selected::Burn { entity: _, time: _, state } = &mut view.selected {
            if state.is_selected() {
                *state = BurnState::Adjusting;
            } else if state.is_adjusting() {
                *state = BurnState::Selected;
            }
        } else {
            view.selected = Selected::Burn { entity: self.entity, time: self.time, state: BurnState::Selected }
        }
    }
}