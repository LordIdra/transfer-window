use std::{cmp::Ordering, fmt::Debug};

use eframe::egui::{PointerState, Pos2, Vec2};
use encounter::Encounter;
use intercept::Intercept;
use nalgebra_glm::DVec2;

use crate::game::{util::{add_textured_square, add_textured_square_facing}, View};

use self::{adjust_burn::AdjustBurn, adjust_fire_torpedo::AdjustFireTorpedo, apsis::Apsis, burn::Burn, closest_approach::ClosestApproach, fire_torpedo::FireTorpedo, guidance::Guidance, orbitable::Orbitable, vessel::Vessel};

mod adjust_burn;
mod adjust_fire_torpedo;
mod apsis;
mod burn;
mod closest_approach;
mod encounter;
mod fire_torpedo;
mod guidance;
mod intercept;
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
    fn texture(&self, view: &View) -> String;
    fn alpha(&self, view: &View, is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32;
    fn radius(&self, view: &View) -> f64;
    fn priorities(&self, view: &View) -> [u64; 4];
    fn position(&self, view: &View) -> DVec2;
    fn facing(&self, view: &View) -> Option<DVec2>;
    fn is_selected(&self, view: &View) -> bool;
    fn on_mouse_over(&self, view: &mut View, pointer: &PointerState);
    fn selectable(&self) -> bool;
    
    fn on_scroll(&self, _view: &mut View, _scroll_delta: Vec2) -> bool { false }

    fn is_hovered(&self, view: &mut View, mouse_position_window: Pos2) -> bool {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Is icon hovered");
        let mouse_position_world = view.window_space_to_world_space(mouse_position_window);
        let radius = self.radius(view) / view.camera.zoom();
        (self.position(view) - mouse_position_world).magnitude() < radius
    }

    fn overlaps(&self, view: &View, other_icon: &dyn Icon) -> bool {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Icon overlaps");
        let min_distance_between_icons = (self.radius(view) + other_icon.radius(view)) / view.camera.zoom();
        (self.position(view) - other_icon.position(view)).magnitude() < min_distance_between_icons
    }

    fn cmp(&self, view: &View, other: &dyn Icon) -> Ordering {
        let priorities = self.priorities(view);
        let other_priorities = other.priorities(view);
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

fn compute_initial_icons(view: &mut View, pointer: &PointerState) -> Vec<Box<dyn Icon>> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Compute initial icons");

    let mut icons: Vec<Box<dyn Icon>> = vec![];
    icons.append(&mut AdjustBurn::generate(view, pointer));
    icons.append(&mut AdjustFireTorpedo::generate(view, pointer));
    icons.append(&mut Apsis::generate(view));
    icons.append(&mut Burn::generate(view));
    icons.append(&mut ClosestApproach::generate(view));
    icons.append(&mut Encounter::generate(view));
    icons.append(&mut FireTorpedo::generate(view));
    icons.append(&mut Guidance::generate(view));
    icons.append(&mut Intercept::generate(view));
    icons.append(&mut Orbitable::generate(view));
    icons.append(&mut Vessel::generate(view));
    icons
}

/// Assumes the input is sorted already from highest to lowest
/// Continually adds the highest priority remaining icon if it does not overlap any icons already added
/// This means that when icons overlap, only the one with the highest priority is added
/// Returns (`not_overlapped`, `overlapped`)
#[allow(clippy::type_complexity)]
fn split_overlapping_icons(view: &View, icons: Vec<Box<dyn Icon>>) -> (Vec<Box<dyn Icon>>, Vec<Box<dyn Icon>>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Split overlapping icons");

    let mut not_overlapped = vec![];
    let mut overlapped = vec![];
    for icon in icons {
        #[allow(clippy::borrowed_box)] // false positive
        let overlaps_any_not_overlapped = not_overlapped.iter().any(|new_icon: &Box<dyn Icon>| new_icon.overlaps(view, &*icon));
        if overlaps_any_not_overlapped {
            overlapped.push(icon);
        } else {
            not_overlapped.push(icon);
        }
    }
    (not_overlapped, overlapped)
}

fn draw_icon(view: &mut View, mouse_position_window: Option<Pos2>, icon: &dyn Icon, is_overlapped: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw icon");

    let radius = icon.radius(view) / view.camera.zoom();
    let is_selected = icon.is_selected(view);
    let is_hovered = if let Some(mouse_position_window) = mouse_position_window{
        icon.is_hovered(view, mouse_position_window)
    } else {
        false
    };
    let alpha = icon.alpha(view, is_selected, is_hovered, is_overlapped);
    if alpha == 0.0 {
        return;
    }

    let mut vertices = vec![];
    if let Some(facing) = icon.facing(view) {
        add_textured_square_facing(&mut vertices, icon.position(view), radius, alpha, facing);
    } else {
        add_textured_square(&mut vertices, icon.position(view), radius, alpha);
    }

    view.renderers.add_texture_vertices(&icon.texture(view), &mut vertices);
}

// 'rust is simple' said no one ever
fn compute_mouse_over_icon<'a>(view: &mut View, mouse_position: Pos2, icons: &'a Vec<Box<dyn Icon>>) -> Option<&'a dyn Icon> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Compute mouse over icon");

    let focus = view.window_space_to_world_space(mouse_position);
    for icon in icons {
        let radius = icon.radius(view) / view.camera.zoom();
        if icon.selectable() && (icon.position(view) - focus).magnitude() < radius {
            return Some(&**icon)
        }
    }
    None
}

/// Returns true if any icon is hovered over
pub fn draw(view: &mut View) -> bool {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw icons");
    let mut any_icon_hovered = false;
    view.icon_captured_scroll = false;
    view.context.clone().input(|input| {
        let mut icons = compute_initial_icons(view, &input.pointer);
        icons.sort_by(|a, b| a.cmp(view, &**b));
        icons.reverse(); // sorted from HIGHEST to LOWEST priority
        
        let (not_overlapped, overlapped) = split_overlapping_icons(view, icons);
        if let Some(mouse_position_window) = input.pointer.latest_pos() {
            if !view.pointer_over_ui {
                if let Some(icon) = compute_mouse_over_icon(view, mouse_position_window, &not_overlapped) {
                    any_icon_hovered = true;
                    icon.on_mouse_over(view, &input.pointer);
                    let scroll_delta = input.smooth_scroll_delta;
                    if (scroll_delta.y != 0.0 || scroll_delta.x != 0.0) && icon.on_scroll(view, scroll_delta) {
                        view.icon_captured_scroll = true;
                    }
                }
            }
        }

        for icon in overlapped {
            draw_icon(view, input.pointer.latest_pos(), &*icon, true);
        }

        for icon in not_overlapped {
            draw_icon(view, input.pointer.latest_pos(), &*icon, false);
        }
    });

    any_icon_hovered
}
