use eframe::egui::Rgba;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::trajectory_component::burn::Burn, storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::Selected, Scene};

use super::Icon;

const OFFSET: f64 = 25.0;

#[derive(Debug)]
pub struct AdjustBurn {
    entity: Entity,
    time: f64,
    direction: DVec2,
}

impl AdjustBurn {
    pub fn generate(view: &Scene, model: &Model) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        if let Selected::Burn { entity, time, state } = view.selected.clone() {
            if state.is_adjusting() {
                let burn = model.get_trajectory_component(entity).get_segment_at_time(time).as_burn();
                let time = burn.get_start_point().get_time();
                burn.get_tangent_direction();
                let icon = Self { entity, time, direction: vec2(1.0, 0.0) };
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self { entity, time, direction: vec2(-1.0, 0.0) };
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self { entity, time, direction: vec2(0.0, 1.0) };
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self { entity, time, direction: vec2(0.0, -1.0) };
                icons.push(Box::new(icon) as Box<dyn Icon>);
            }
        }
        icons
    }

    fn get_burn<'a>(&self, model: &'a Model) -> &'a Burn {
        model.get_trajectory_component(self.entity).get_segment_at_time(self.time).as_burn()
    }
}

impl Icon for AdjustBurn {
    fn get_texture(&self) -> &str {
        "adjust-burn-arrow"
    }

    fn get_color(&self) -> eframe::egui::Rgba {
        Rgba::from_rgb(0.4, 0.8, 1.0)
    }

    fn get_radius(&self) -> f64 {
        10.0
    }

    fn get_priorities(&self, _view: &Scene, _model: &Model) -> [u64; 4] {
        [1, 0, 0, 0]
    }

    fn get_position(&self, view: &Scene, model: &Model) -> DVec2 {
        let burn = self.get_burn(model);
        let offset = OFFSET * burn.get_rotation_matrix() * self.direction / view.camera.get_zoom();
        model.get_absolute_position(burn.get_parent()) + burn.get_start_point().get_position() + offset
    }

    fn get_facing(&self, _view: &Scene, model: &Model) -> Option<DVec2> {
        let burn = self.get_burn(model);
        Some(burn.get_rotation_matrix() * self.direction)
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        #[allow(clippy::float_cmp)] // time and self.time should be *exactly* the same
        match &view.selected {
            Selected::None | Selected::Point { entity: _, time: _, state: _ } => false,
            Selected::Burn { entity, time, state: _ } => *entity == self.entity && *time == self.time,
        }
    }

    fn on_clicked(&self, _view: &mut Scene, _model: &Model) {
        
    }
}
