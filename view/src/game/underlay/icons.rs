use std::{cmp::Ordering, fmt::Debug};

use eframe::egui::{Context, Pos2, Rect, Rgba};
use log::error;
use nalgebra_glm::DVec2;
use transfer_window_model::Model;

use crate::game::{util::add_textured_square, Scene};

use self::{burn::Burn, orbitable::Orbitable, vessel::Vessel};

const ICON_RADIUS: f64 = 10.0;
const REGULAR_ALPHA: f32 = 0.6;
const HOVERED_ALPHA: f32 = 0.8;
const SELECTED_ALPHA: f32 = 1.0;

mod burn;
mod orbitable;
mod vessel;

trait Icon: Debug {
    fn get_texture(&self) -> &str;
    fn get_color(&self) -> Rgba;
    fn get_group_priority(&self) -> usize;
    fn get_priority(&self, model: &Model) -> usize;
    fn get_position(&self, model: &Model) -> DVec2;
    fn is_selected(&self, view: &Scene) -> bool;
    fn on_clicked(&self, view: &mut Scene);

    fn is_hovered(&self, view: &Scene, model: &Model, mouse_position_window: Pos2, screen_size: Rect) -> bool {
        let mouse_position_world = view.camera.window_space_to_world_space(model, mouse_position_window, screen_size);
        let radius = ICON_RADIUS / view.camera.get_zoom();
        (self.get_position(model) - mouse_position_world).magnitude() < radius
    }

    fn overlaps(&self, view: &Scene, model: &Model, other_icon: &dyn Icon) -> bool {
        let min_distance_between_icons = 2.05 * ICON_RADIUS / view.camera.get_zoom();
        (self.get_position(model) - other_icon.get_position(model)).magnitude() < min_distance_between_icons
    }

    fn cmp(&self, view: &Scene, model: &Model, other: &dyn Icon) -> Ordering {
        if self.is_selected(view) {
            Ordering::Greater
        } else if other.is_selected(view) {
            Ordering::Less
        } else if self.get_group_priority() == other.get_group_priority() {
            self.get_priority(model).cmp(&other.get_priority(model))
        } else {
            self.get_group_priority().cmp(&other.get_group_priority())
        }
    }
}

fn get_initial_icons(model: &Model) -> Vec<Box<dyn Icon>> {
    let mut icons: Vec<Box<dyn Icon>> = vec![];
    icons.append(&mut Vessel::generate(model));
    icons.append(&mut Orbitable::generate(model));
    icons.append(&mut Burn::generate(model));
    icons
}

/// Assumes the input is sorted already from highest to lowest
/// Continually adds the highest priority remaining icon if it does not overlap any icons already added
/// This means that when icons overlap, only the one with the highest priority is added
fn remove_overlapping_icons(view: &Scene, model: &Model, icons: Vec<Box<dyn Icon>>) -> Vec<Box<dyn Icon>> {
    let mut new_icons = vec![];
    for icon in icons {
        #[allow(clippy::borrowed_box)] // false positive
        if !new_icons.iter().any(|new_icon: &Box<dyn Icon>| new_icon.overlaps(view, model, &*icon)) {
            new_icons.push(icon);
        }
    }
    new_icons
}

fn get_alpha(view: &Scene, model: &Model, icon: &dyn Icon, mouse_position_window: Option<Pos2>, screen_size: Rect) -> f32 {
    if icon.is_selected(view) {
        return SELECTED_ALPHA;
    }
    if let Some(mouse_position_window) = mouse_position_window {
        if icon.is_hovered(view, model, mouse_position_window, screen_size) {
            return HOVERED_ALPHA;
        }
    }
    REGULAR_ALPHA
}

fn draw_icons(view: &Scene, model: &Model, icons: &Vec<Box<dyn Icon>>, mouse_position_window: Option<Pos2>, screen_size: Rect) {
    let radius = ICON_RADIUS / view.camera.get_zoom();
    for icon in icons {
        let mut vertices = vec![];
        let alpha = get_alpha(view, model, &**icon, mouse_position_window, screen_size);
        let color = icon.get_color().multiply(alpha);
        add_textured_square(&mut vertices, icon.get_position(model), radius, color);
        let Some(texture_renderer) = view.texture_renderers.get(icon.get_texture()) else {
            error!("Texture {} does not exist ", icon.get_texture());
            continue;
        };
        texture_renderer.lock().unwrap().add_vertices(&mut vertices);
    }
}

// 'rust is simple' said no one ever
fn get_mouse_over_icon<'a>(view: &Scene, model: &Model, mouse_position: Pos2, screen_size: Rect, icons: &'a mut Vec<Box<dyn Icon>>) -> Option<&'a mut dyn Icon> {
    let focus = view.camera.window_space_to_world_space(model, mouse_position, screen_size);
    let radius = ICON_RADIUS / view.camera.get_zoom();
    for icon in icons {
        if (icon.get_position(model) - focus).magnitude() < radius {
            return Some(&mut **icon)
        }
    }
    None
}

/// Returns true if any icon is hovered over
pub fn draw(view: &mut Scene, model: &Model, context: &Context) -> bool {
    let mut icons = get_initial_icons(model);
    icons.sort_by(|a, b| a.cmp(view, model, &**b));
    icons.reverse(); // sorted from HIGHEST to LOWEST priority
    icons = remove_overlapping_icons(view, model, icons);

    let mut any_icon_hovered = false;
    context.input(|input| {
        if let Some(mouse_position_window) = input.pointer.latest_pos() {
            if let Some(icon) = get_mouse_over_icon(view, model, mouse_position_window, context.screen_rect(), &mut icons) {
                any_icon_hovered = true;
                if input.pointer.primary_clicked() {
                    icon.on_clicked(view);
                }
            }
        }

        draw_icons(view, model, &icons, input.pointer.latest_pos(), context.screen_rect());
    });


    any_icon_hovered
}
