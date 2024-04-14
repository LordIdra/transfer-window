use std::{cmp::Ordering, fmt::Debug};

use eframe::egui::{Context, PointerState, Pos2, Rect};
use log::error;
use nalgebra_glm::DVec2;
use transfer_window_model::Model;

use crate::game::{util::{add_textured_square, add_textured_square_facing}, Scene};

use self::{adjust_burn::AdjustBurn, burn::Burn, burn_locked::BurnLocked, orbitable::Orbitable, vessel::Vessel};

mod adjust_burn;
mod burn_locked;
mod burn;
mod orbitable;
mod vessel;

/// The icon trait represents a single 'type' of icon
/// # Priority
/// Icons are sorted based on a priority system with
/// multiple levels of priority. Priorities are
/// compared from the highest priority level until
/// thereis a mismatch in priorities, at which point
/// the icon with the highest priority at that level 
/// is chosen.
trait Icon: Debug {
    fn get_texture(&self) -> &str;
    fn get_alpha(&self, view: &Scene, model: &Model, is_selected: bool, is_hovered: bool) -> f32;
    fn get_radius(&self) -> f64;
    fn get_priorities(&self, view: &Scene, model: &Model) -> [u64; 4];
    fn get_position(&self, view: &Scene, model: &Model) -> DVec2;
    fn get_facing(&self, view: &Scene, model: &Model) -> Option<DVec2>;
    fn is_selected(&self, view: &Scene, model: &Model) -> bool;
    fn on_mouse_over(&self, view: &mut Scene, model: &Model, pointer: &PointerState);

    fn is_hovered(&self, view: &Scene, model: &Model, mouse_position_window: Pos2, screen_size: Rect) -> bool {
        let mouse_position_world = view.camera.window_space_to_world_space(model, mouse_position_window, screen_size);
        let radius = self.get_radius() / view.camera.get_zoom();
        (self.get_position(view, model) - mouse_position_world).magnitude() < radius
    }

    fn overlaps(&self, view: &Scene, model: &Model, other_icon: &dyn Icon) -> bool {
        let min_distance_between_icons = 2.05 * self.get_radius() / view.camera.get_zoom();
        (self.get_position(view, model) - other_icon.get_position(view, model)).magnitude() < min_distance_between_icons
    }

    fn cmp(&self, view: &Scene, model: &Model, other: &dyn Icon) -> Ordering {
        let priorities = self.get_priorities(view, model);
        let other_priorities = other.get_priorities(view, model);
        if priorities[0] != other_priorities[0] {
            priorities[0].cmp(&other_priorities[0])
        } else if priorities[1] != other_priorities[1] {
            priorities[1].cmp(&other_priorities[1])
        } else if priorities[2] != other_priorities[2] {
            priorities[2].cmp(&other_priorities[2])
        } else if priorities[3] != other_priorities[3] {
            priorities[3].cmp(&other_priorities[3])
        } else {
            Ordering::Equal
        }
    }
}

fn get_initial_icons(view: &Scene, model: &Model, pointer: &PointerState, screen_rect: Rect) -> Vec<Box<dyn Icon>> {
    let mut icons: Vec<Box<dyn Icon>> = vec![];
    icons.append(&mut AdjustBurn::generate(view, model, pointer, screen_rect));
    icons.append(&mut BurnLocked::generate(model));
    icons.append(&mut Burn::generate(model));
    icons.append(&mut Orbitable::generate(model));
    icons.append(&mut Vessel::generate(model));
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

fn draw_icons(view: &Scene, model: &Model, icons: &Vec<Box<dyn Icon>>, mouse_position_window: Option<Pos2>, screen_size: Rect) {
    for icon in icons {
        let radius = icon.get_radius() / view.camera.get_zoom();
        let is_selected = icon.is_selected(view, model);
        let is_hovered = if let Some(mouse_position_window) = mouse_position_window{
            icon.is_hovered(view, model, mouse_position_window, screen_size)
        } else {
            false
        };
        let alpha = icon.get_alpha(view, model, is_selected, is_hovered);

        let mut vertices = vec![];
        if let Some(facing) = icon.get_facing(view, model) {
            add_textured_square_facing(&mut vertices, icon.get_position(view, model), radius, alpha, facing);
        } else {
            add_textured_square(&mut vertices, icon.get_position(view, model), radius, alpha);
        }
        
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
    for icon in icons {
        let radius = icon.get_radius() / view.camera.get_zoom();
        if (icon.get_position(view, model) - focus).magnitude() < radius {
            return Some(&mut **icon)
        }
    }
    None
}

/// Returns true if any icon is hovered over
pub fn draw(view: &mut Scene, model: &Model, context: &Context) -> bool {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw icons");
    let mut any_icon_hovered = false;
    context.input(|input| {
        let mut icons = get_initial_icons(view, model, &input.pointer, context.screen_rect());
        icons.sort_by(|a, b| a.cmp(view, model, &**b));
        icons.reverse(); // sorted from HIGHEST to LOWEST priority
        icons = remove_overlapping_icons(view, model, icons);

        if let Some(mouse_position_window) = input.pointer.latest_pos() {
            if let Some(icon) = get_mouse_over_icon(view, model, mouse_position_window, context.screen_rect(), &mut icons) {
                any_icon_hovered = true;
                icon.on_mouse_over(view, model, &input.pointer);
            }
        }

        draw_icons(view, model, &icons, input.pointer.latest_pos(), context.screen_rect());
    });

    any_icon_hovered
}
