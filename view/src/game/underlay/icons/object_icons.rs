use std::cmp::Ordering;

use eframe::egui::{Context, Pos2, Rect, Rgba};
use log::error;
use nalgebra_glm::DVec2;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::{util::add_textured_square, Scene};

const ICON_RADIUS: f64 = 10.0;

#[derive(Debug, Clone)]
struct Icon {
    entity: Entity,
    group_priority: usize, // what 'group' of icons this belongs to (ship, planet, torpedo, etc)
    priority: usize, // how this icon should be ordered within its group
    texture: String,
    position: DVec2,
}

impl Icon {
    pub fn new(entity: Entity, group_priority: usize, priority: usize, texture: String, position: DVec2) -> Self {
        Self { entity, group_priority, priority, texture, position }
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
}

fn get_icon(model: &Model, entity: Entity) -> Option<Icon> {
    model.get_position(entity)?; // make sure the entity actually has a position, otherwise next line will panic
    let position = model.get_absolute_position(entity);

    if model.try_get_orbitable_component(entity).is_some() {
        // We divide here to make sure the usize doesn't overflow
        #[allow(clippy::cast_sign_loss)]
        let priority = (model.get_mass_component(entity).get_mass() / 1.0e22) as usize;
        return Some(Icon::new(entity, 2, priority, "planet".to_string(), position));
    }

    None
}

fn get_initial_icons(model: &Model) -> Vec<Icon> {
    let mut icons = vec![];
    for entity in model.get_entities(vec![]) {
        if let Some(icon) = get_icon(model, entity) {
            icons.push(icon);
        }
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

fn draw_icons(view: &Scene, icons: Vec<Icon>) {
    let radius = ICON_RADIUS / view.camera.get_zoom();
    for icon in icons {
        let mut vertices = vec![];
        add_textured_square(&mut vertices, icon.position, radius, Rgba::WHITE);
        let Some(texture_renderer) = view.texture_renderers.get(&icon.texture) else {
            error!("Texture {} does not exist ", icon.texture);
            continue;
        };
        texture_renderer.lock().unwrap().add_vertices(&mut vertices);
    }
}

fn get_mouse_over_icon(view: &Scene, model: &Model, mouse_position: Pos2, screen_size: Rect, icons: &[Icon]) -> Option<Icon> {
    let focus = view.camera.window_space_to_world_space(model, mouse_position, screen_size);
    let radius = ICON_RADIUS / view.camera.get_zoom();
    icons.iter().find(|icon| (icon.position - focus).magnitude() < radius).cloned()
}

pub fn draw(view: &mut Scene, model: &Model, context: &Context) {
    let mut icons = get_initial_icons(model);
    icons.sort_by(Icon::cmp);
    icons.reverse(); // sorted from HIGHEST to LOWEST priority
    icons = remove_overlapping_icons(view, icons);
    context.input(|input| {
        if !input.pointer.primary_clicked() {
            return;
        }
        let Some(latest_position) = input.pointer.latest_pos() else {
            return;
        };
        if let Some(icon) = get_mouse_over_icon(view, model, latest_position, context.screen_rect(), &icons) {
            view.camera.reset_panning();
            view.camera.set_focus(Some(icon.entity));
        }
    });
    draw_icons(view, icons);
}

// - global mutable state, mutable state, and state should be avoided when possible (in that order)
// - OOP works well on some problems (especially 'real life' problems) but tends to break down when dealing with more abstract or complex stuff (eg databases can be absolute hell to work with, solving abstract problems often ends with inventing weird abstractions)
// - diagrams to draw some high-level architecture or explain something complex are fantastic, having an idea of what you're building and why is fantastic, but fully fledged UML diagrams are usually a bad idea
// - commenting is helpful when done right (eg explaining why a line of code does something you might not expect) but worse than no comments when done wrong (eg '// Build the window' above 'buildWindow()')
// - documentation is helpful when trying to understand a project you haven't worked on, but often not worth the cost to maintain, especially in rapidly-changing codebases. it's usually much more obvious when your code is doing something wrong than when your docs are wrong. if you're going to do docs, do them at the end on components that are unlikely to change for a while, don't do them on literally everything as you go along, ever
// - logging - again helpful when done right, but not necessary or useful in many projects. in TetrECS I've been quite thoroughly logging and have yet to find the log output helpful when debugging
// - abstraction is a double edged sword, can sometimes make things simpler, can sometimes make you jump through 438974389 method calls to find out what the hell is going on, be VERY careful about what abstractions you make
// - this ties into abstraction, making code 'simple' is generally more important than making it 'easy'
// - single responsibility of methods is good, but single responsibility of code units (files, modules, packages, whatever) is usually understated, it's more important in practice IMO and allows for much better abstractions
