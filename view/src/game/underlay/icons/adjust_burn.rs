use eframe::egui::{PointerState, Rect, Rgba};
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::trajectory_component::burn::Burn, storage::entity_allocator::Entity, Model};

use crate::game::{underlay::selected::{burn::{BurnAdjustDirection, BurnState}, Selected}, util::get_burn_arrow_position, Scene};

use super::Icon;

fn offset(amount: f64) -> f64 {
    if amount.is_sign_positive() {
        0.2 * amount
    } else {
        0.02 * amount
    }
}

#[derive(Debug)]
pub struct AdjustBurn {
    entity: Entity,
    time: f64,
    position: DVec2,
    direction: BurnAdjustDirection,
}

impl AdjustBurn {
    fn new(view: &Scene, model: &Model, entity: Entity, time: f64, direction: BurnAdjustDirection, pointer: &PointerState, screen_rect: Rect) -> Self {
        let burn = model.get_trajectory_component(entity).get_last_segment_at_time(time).as_burn();
        let burn_to_arrow_unit = burn.get_rotation_matrix() * direction.get_vector();
        let mut position = get_burn_arrow_position(view, model, entity, time, &direction);

        // Additional offset if arrow is being dragged
        if let Some(mouse_position) = pointer.latest_pos() {
            if let Selected::Burn { entity: _, time: _, state: BurnState::Dragging(drag_direction) } = &view.selected {
                if *drag_direction == direction {
                    let arrow_to_mouse = view.camera.window_space_to_world_space(model, mouse_position, screen_rect) - position;
                    let amount = arrow_to_mouse.dot(&burn_to_arrow_unit);
                    position += offset(amount) * burn_to_arrow_unit;
                }
            }
        }

        Self { entity, time, position, direction }
    }

    pub fn generate(view: &Scene, model: &Model, pointer: &PointerState, screen_rect: Rect) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        if let Selected::Burn { entity, time, state } = view.selected.clone() {
            if state.is_adjusting() || state.is_dragging() {
                let burn = model.get_trajectory_component(entity).get_last_segment_at_time(time).as_burn();
                let time = burn.get_start_point().get_time();
                burn.get_tangent_direction();
                let icon = Self::new(view, model, entity, time, BurnAdjustDirection::Prograde, pointer, screen_rect);
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self::new(view, model, entity, time, BurnAdjustDirection::Retrograde, pointer, screen_rect);
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self::new(view, model, entity, time, BurnAdjustDirection::Normal, pointer, screen_rect);
                icons.push(Box::new(icon) as Box<dyn Icon>);
                let icon = Self::new(view, model, entity, time, BurnAdjustDirection::Antinormal, pointer, screen_rect);
                icons.push(Box::new(icon) as Box<dyn Icon>);
            }
        }
        icons
    }

    fn get_burn<'a>(&self, model: &'a Model) -> &'a Burn {
        model.get_trajectory_component(self.entity).get_last_segment_at_time(self.time).as_burn()
    }
}

impl Icon for AdjustBurn {
    fn get_texture(&self) -> &str {
        "adjust-burn-arrow"
    }

    fn get_color(&self, view: &Scene) -> eframe::egui::Rgba {
        if let Selected::Burn { entity: _, time: _, state: BurnState::Dragging(direction) } = &view.selected {
            if *direction == self.direction {
                return Rgba::from_rgba_unmultiplied(0.4, 0.8, 1.0, 1.0)
            }
            // Dim the other arrows if we are dragging one
            return Rgba::from_rgba_unmultiplied(0.4, 0.8, 1.0, 0.8)
        };
        Rgba::from_rgba_unmultiplied(0.4, 0.8, 1.0, 1.0)
    }

    fn get_radius(&self) -> f64 {
        10.0
    }

    fn get_priorities(&self, _view: &Scene, _model: &Model) -> [u64; 4] {
        [2, 0, 0, 0]
    }

    fn get_position(&self, _view: &Scene, _model: &Model, ) -> DVec2 {
        self.position
    }

    fn get_facing(&self, _view: &Scene, model: &Model) -> Option<DVec2> {
        let burn = self.get_burn(model);
        Some(burn.get_rotation_matrix() * self.direction.get_vector())
    }

    fn is_selected(&self, view: &Scene, _model: &Model) -> bool {
        if let Selected::Burn { entity: _, time: _, state: BurnState::Dragging(direction) } = &view.selected {
            return *direction == self.direction
        }
        false
    }

    fn on_mouse_over(&self, view: &mut Scene, _model: &Model, pointer: &PointerState) {
        if let Selected::Burn { entity: _, time: _, state } = &mut view.selected {
            if pointer.primary_clicked() {
                trace!("Started dragging to adjust burn {:?}", self.direction);
                *state = BurnState::Dragging(self.direction.clone());
            }
        }
    }
}
