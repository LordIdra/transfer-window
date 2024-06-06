use std::{cmp::Ordering, fmt::Debug};

use eframe::egui::{Context, PointerState, Pos2, Rect, Vec2};
use encounter::Encounter;
use intercept::Intercept;
use nalgebra_glm::DVec2;
use transfer_window_model::Model;

use crate::game::{util::{add_textured_square, add_textured_square_facing}, Scene};

use self::{adjust_burn::AdjustBurn, adjust_fire_torpedo::AdjustFireTorpedo, apoapsis::Apoapsis, burn::Burn, closest_approach::ClosestApproach, fire_torpedo::FireTorpedo, guidance::Guidance, orbitable::Orbitable, periapsis::Periapsis, vessel::Vessel};

mod adjust_burn;
mod adjust_fire_torpedo;
mod apoapsis;
mod burn;
mod closest_approach;
mod encounter;
mod fire_torpedo;
mod guidance;
mod intercept;
mod orbitable;
mod periapsis;
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
    fn texture(&self, view: &Scene, model: &Model) -> String;
    fn alpha(&self, view: &Scene, model: &Model, is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32;
    fn radius(&self, view: &Scene, model: &Model) -> f64;
    fn priorities(&self, view: &Scene, model: &Model) -> [u64; 4];
    fn position(&self, view: &Scene, model: &Model) -> DVec2;
    fn facing(&self, view: &Scene, model: &Model) -> Option<DVec2>;
    fn is_selected(&self, view: &Scene, model: &Model) -> bool;
    fn on_mouse_over(&self, view: &mut Scene, model: &Model, pointer: &PointerState);
    fn selectable(&self) -> bool;
    
    fn on_scroll(&self, _view: &mut Scene, _model: &Model, _scroll_delta: Vec2) -> bool { false }

    fn is_hovered(&self, view: &mut Scene, model: &Model, mouse_position_window: Pos2, screen_size: Rect) -> bool {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Is icon hovered");
        let mouse_position_world = view.camera.window_space_to_world_space(model, mouse_position_window, screen_size);
        let radius = self.radius(view, model) / view.camera.zoom();
        (self.position(view, model) - mouse_position_world).magnitude() < radius
    }

    fn overlaps(&self, view: &Scene, model: &Model, other_icon: &dyn Icon) -> bool {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Icon overlaps");
        let min_distance_between_icons = (self.radius(view, model) + other_icon.radius(view, model)) / view.camera.zoom();
        (self.position(view, model) - other_icon.position(view, model)).magnitude() < min_distance_between_icons
    }

    fn cmp(&self, view: &Scene, model: &Model, other: &dyn Icon) -> Ordering {
        let priorities = self.priorities(view, model);
        let other_priorities = other.priorities(view, model);
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

fn compute_initial_icons(view: &mut Scene, model: &Model, pointer: &PointerState, screen_rect: Rect) -> Vec<Box<dyn Icon>> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Compute initial icons");

    let mut icons: Vec<Box<dyn Icon>> = vec![];
    icons.append(&mut AdjustBurn::generate(view, model, pointer, screen_rect));
    icons.append(&mut AdjustFireTorpedo::generate(view, model, pointer, screen_rect));
    icons.append(&mut Apoapsis::generate(view, model));
    icons.append(&mut Burn::generate(view, model));
    icons.append(&mut ClosestApproach::generate(view, model));
    icons.append(&mut Encounter::generate(view, model));
    icons.append(&mut FireTorpedo::generate(view, model));
    icons.append(&mut Guidance::generate(view, model));
    icons.append(&mut Intercept::generate(view, model));
    icons.append(&mut Orbitable::generate(view, model));
    icons.append(&mut Periapsis::generate(view, model));
    icons.append(&mut Vessel::generate(view, model));
    icons
}

/// Assumes the input is sorted already from highest to lowest
/// Continually adds the highest priority remaining icon if it does not overlap any icons already added
/// This means that when icons overlap, only the one with the highest priority is added
/// Returns (`not_overlapped`, `overlapped`)
#[allow(clippy::type_complexity)]
fn split_overlapping_icons(view: &Scene, model: &Model, icons: Vec<Box<dyn Icon>>) -> (Vec<Box<dyn Icon>>, Vec<Box<dyn Icon>>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Split overlapping icons");

    let mut not_overlapped = vec![];
    let mut overlapped = vec![];
    for icon in icons {
        #[allow(clippy::borrowed_box)] // false positive
        let overlaps_any_not_overlapped = not_overlapped.iter().any(|new_icon: &Box<dyn Icon>| new_icon.overlaps(view, model, &*icon));
        if overlaps_any_not_overlapped {
            overlapped.push(icon);
        } else {
            not_overlapped.push(icon);
        }
    }
    (not_overlapped, overlapped)
}

fn draw_icon(view: &mut Scene, model: &Model, mouse_position_window: Option<Pos2>, screen_size: Rect, icon: &dyn Icon, is_overlapped: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw icon");

    let radius = icon.radius(view, model) / view.camera.zoom();
    let is_selected = icon.is_selected(view, model);
    let is_hovered = if let Some(mouse_position_window) = mouse_position_window{
        icon.is_hovered(view, model, mouse_position_window, screen_size)
    } else {
        false
    };
    let alpha = icon.alpha(view, model, is_selected, is_hovered, is_overlapped);
    if alpha == 0.0 {
        return;
    }

    let mut vertices = vec![];
    if let Some(facing) = icon.facing(view, model) {
        add_textured_square_facing(&mut vertices, icon.position(view, model), radius, alpha, facing);
    } else {
        add_textured_square(&mut vertices, icon.position(view, model), radius, alpha);
    }

    view.renderers.add_texture_vertices(&icon.texture(view, model), &mut vertices);
}

// 'rust is simple' said no one ever
fn compute_mouse_over_icon<'a>(view: &mut Scene, model: &Model, mouse_position: Pos2, screen_size: Rect, icons: &'a Vec<Box<dyn Icon>>) -> Option<&'a dyn Icon> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Compute mouse over icon");

    let focus = view.camera.window_space_to_world_space(model, mouse_position, screen_size);
    for icon in icons {
        let radius = icon.radius(view, model) / view.camera.zoom();
        if icon.selectable() && (icon.position(view, model) - focus).magnitude() < radius {
            return Some(&**icon)
        }
    }
    None
}

/// Returns true if any icon is hovered over
pub fn draw(view: &mut Scene, model: &Model, context: &Context) -> bool {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw icons");
    let mut any_icon_hovered = false;
    view.icon_captured_scroll = false;
    let screen_rect = context.screen_rect();
    context.input(|input| {
        let mut icons = compute_initial_icons(view, model, &input.pointer, screen_rect);
        icons.sort_by(|a, b| a.cmp(view, model, &**b));
        icons.reverse(); // sorted from HIGHEST to LOWEST priority
        
        let (not_overlapped, overlapped) = split_overlapping_icons(view, model, icons);
        if let Some(mouse_position_window) = input.pointer.latest_pos() {
            if !view.pointer_over_ui_last_frame {
                if let Some(icon) = compute_mouse_over_icon(view, model, mouse_position_window, screen_rect, &not_overlapped) {
                    any_icon_hovered = true;
                    icon.on_mouse_over(view, model, &input.pointer);
                    let scroll_delta = input.smooth_scroll_delta;
                    if (scroll_delta.y != 0.0 || scroll_delta.x != 0.0) && icon.on_scroll(view, model, scroll_delta) {
                        view.icon_captured_scroll = true;
                    }
                }
            }
        }

        for icon in overlapped {
            draw_icon(view, model, input.pointer.latest_pos(), screen_rect, &*icon, true);
        }

        for icon in not_overlapped {
            draw_icon(view, model, input.pointer.latest_pos(), screen_rect, &*icon, false);
        }
    });

    any_icon_hovered
}
