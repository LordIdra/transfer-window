use std::cmp::Ordering;

use eframe::egui::{Context, Pos2, Rect, Rgba};
use log::error;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::trajectory_component::{segment::Segment, TrajectoryComponent}, storage::entity_allocator::Entity, Model};

use crate::game::{util::add_textured_square, Scene};

use super::selected::Selected;

const ICON_RADIUS: f64 = 10.0;

#[derive(Debug, Clone)]
enum IconType {
    Orbitable,
    Vessel,
    Burn,
}

impl IconType {
    pub fn get_texture(&self) -> &str {
        match self {
            IconType::Orbitable => "planet",
            IconType::Vessel => "spacecraft",
            IconType::Burn => "burn",
        }
    }

    pub fn get_color(&self) -> Rgba {
        match self {
            IconType::Orbitable => Rgba::from_rgb(1.0, 1.0, 1.0),
            IconType::Vessel => Rgba::from_rgb(0.7, 0.85, 1.0),
            IconType::Burn => Rgba::from_rgb(1.0, 0.7, 0.5),
        }
    }

    pub fn on_clicked(&self, view: &mut Scene, entity: Entity) {
        match self {
            IconType::Orbitable => {
                view.camera.reset_panning();
                view.camera.set_focus(Some(entity));
            },
            IconType::Vessel => todo!(),
            IconType::Burn => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
enum IconState {
    None,
    Hovered,
    Selected,
}

impl IconState {
    pub fn get_alpha_multiplier(&self) -> f32 {
        match self {
            IconState::None => 0.6,
            IconState::Hovered => 0.8,
            IconState::Selected => 1.0,
        }
    }
}

#[derive(Debug)]
struct Icon {
    entity: Entity,
    group_priority: usize, // what 'group' of icons this belongs to (ship, planet, torpedo, etc)
    priority: usize, // how this icon should be ordered within its group
    icon_type: IconType,
    icon_state: IconState,
    position: DVec2,
}

impl Icon {
    pub fn new(entity: Entity, group_priority: usize, priority: usize, icon_type: IconType, icon_state: IconState, position: DVec2) -> Self {
        Self { entity, group_priority, priority, icon_type, icon_state, position }
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        if self.group_priority == other.group_priority {
            self.priority.cmp(&other.priority)
        } else {
            self.group_priority.cmp(&other.group_priority)
        }
    }

    pub fn overlaps(&self, other_icon: &Icon, min_distance_between_icons: f64) -> bool {
        (self.position - other_icon.position).magnitude() < min_distance_between_icons
    }

    pub fn set_state(&mut self, state: IconState) {
        self.icon_state = state;
    }

    pub fn get_state(&self) -> IconState {
        self.icon_state.clone()
    }

    pub fn get_type(&self) -> IconType {
        self.icon_type.clone()
    }
}

fn get_icon(model: &Model, view: &Scene, entity: Entity) -> Vec<Icon> {
    let mut icons = vec![];

    if model.try_get_orbitable_component(entity).is_some() {
        get_icon_orbitable(model, entity, view, &mut icons);
    }

    if model.try_get_vessel_component(entity).is_some() {
        get_icon_vessel(model, entity, view, &mut icons);
    }

    if let Some(trajectory_component) = model.try_get_trajectory_component(entity) {
        get_icon_burn(trajectory_component, model, view, entity, &mut icons);
    }

    icons
}

fn get_icon_burn(trajectory_component: &TrajectoryComponent, model: &Model, view: &Scene, entity: Entity, icons: &mut Vec<Icon>) {
    for segment in trajectory_component.get_segments() {
        if let Some(segment) = segment {
            if let Segment::Burn(burn) = segment {
                let position = model.get_absolute_position(burn.get_parent()) + burn.get_start_point().get_position();
                let state = match &view.selected {
                    Selected::None => IconState::None,
                    Selected::Point { entity: _, time: _, state: _ } => IconState::None,
                    Selected::Burn { entity, time, state: _ } => if *entity == burn.get_entity() && *time == burn.get_start_point().get_time() {
                        IconState::Selected
                    } else {
                        IconState::None
                    },
                };
                let icon = Icon::new(entity, 0, 0, IconType::Burn, state, position);
                icons.push(icon);
            }
        }
    }
}

fn get_icon_vessel(model: &Model, entity: Entity, view: &Scene, icons: &mut Vec<Icon>) {
    // We divide here to make sure the usize doesn't overflow
    #[allow(clippy::cast_sign_loss)]
    let priority = (model.get_mass_component(entity).get_mass() / 1.0e22) as usize;
    if model.get_position(entity).is_some() {
        let position = model.get_absolute_position(entity);
        let state = if let Some(focus) = view.camera.get_focus() {
            if focus == entity {
                IconState::Selected
            } else {
                IconState::None
            }
        } else {
            IconState::None
        };
        let icon = Icon::new(entity, 1, priority, IconType::Vessel, state, position);
        icons.push(icon);
    }
}

fn get_icon_orbitable(model: &Model, entity: Entity, view: &Scene, icons: &mut Vec<Icon>) {
    // We divide here to make sure the usize doesn't overflow
    #[allow(clippy::cast_sign_loss)]
    let priority = (model.get_mass_component(entity).get_mass() / 1.0e22) as usize;
    if model.get_position(entity).is_some() {
        let position = model.get_absolute_position(entity);
        let state = if let Some(focus) = view.camera.get_focus() {
            if focus == entity {
                IconState::Selected
            } else {
                IconState::None
            }
        } else {
            IconState::None
        };
        let icon = Icon::new(entity, 2, priority, IconType::Orbitable, state, position);
        icons.push(icon);
    }
}

fn get_initial_icons(view: &Scene, model: &Model) -> Vec<Icon> {
    let mut icons = vec![];
    for entity in model.get_entities(vec![]) {
        icons.append(&mut get_icon(model, view, entity));
    }
    icons
}

/// Assumes the input is sorted already from highest to lowest
/// Continually adds the highest priority remaining icon if it does not overlap any icons already added
/// This means that when icons overlap, only the one with the highest priority is added
fn remove_overlapping_icons(view: &Scene, icons: Vec<Icon>) -> Vec<Icon> {
    let min_distance_between_icons = 2.05 * ICON_RADIUS / view.camera.get_zoom();
    let mut new_icons = vec![];
    for icon in icons {
        if !new_icons.iter().any(|new_icon: &Icon| new_icon.overlaps(&icon, min_distance_between_icons)) {
            new_icons.push(icon);
        }
    }
    new_icons
}

fn draw_icons(view: &Scene, icons: &Vec<Icon>) {
    let radius = ICON_RADIUS / view.camera.get_zoom();
    for icon in icons {
        let mut vertices = vec![];
        let color = icon.get_type().get_color().multiply(icon.get_state().get_alpha_multiplier());
        add_textured_square(&mut vertices, icon.position, radius, color);
        let Some(texture_renderer) = view.texture_renderers.get(icon.get_type().get_texture()) else {
            error!("Texture {} does not exist ", icon.get_type().get_texture());
            continue;
        };
        texture_renderer.lock().unwrap().add_vertices(&mut vertices);
    }
}

fn get_mouse_over_icon<'a>(view: &Scene, model: &Model, mouse_position: Pos2, screen_size: Rect, icons: &'a mut Vec<Icon>) -> Option<&'a mut Icon> {
    let focus = view.camera.window_space_to_world_space(model, mouse_position, screen_size);
    let radius = ICON_RADIUS / view.camera.get_zoom();
    for icon in icons {
        if (icon.position - focus).magnitude() < radius {
            return Some(icon)
        }
    }
    None
}

/// Returns true if any icon is hovered over
pub fn draw(view: &mut Scene, model: &Model, context: &Context) -> bool {
    let mut icons = get_initial_icons(view, model);
    icons.sort_by(Icon::cmp);
    icons.reverse(); // sorted from HIGHEST to LOWEST priority
    icons = remove_overlapping_icons(view, icons);

    let mut any_icon_clicked = false;
    context.input(|input| {
        let Some(latest_position) = input.pointer.latest_pos() else {
            return;
        };
        if let Some(icon) = get_mouse_over_icon(view, model, latest_position, context.screen_rect(), &mut icons) {
            icon.set_state(if input.pointer.primary_clicked() { IconState::Selected } else { IconState::Hovered });
            if input.pointer.primary_clicked() {
                any_icon_clicked = true;
                icon.get_type().on_clicked(view, icon.entity);
            }
        }
    });

    draw_icons(view, &icons);

    any_icon_clicked
}
