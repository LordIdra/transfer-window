use std::collections::HashSet;

use eframe::egui::{Align2, Color32, RichText, Ui, Window};
use eframe::epaint;
use transfer_window_model::components::vessel_component::faction::Faction;
use transfer_window_model::components::ComponentType;
use transfer_window_model::storage::entity_allocator::Entity;

use super::widgets::custom_image::CustomImage;
use super::widgets::custom_image_button::CustomCircularImageButton;
use super::widgets::labels::draw_title;
use super::View;
use crate::game::events::ViewEvent;
use crate::game::selected::Selected;
use crate::game::util::{orbitable_texture, vessel_texture};

pub fn vessel_hover_circle_color(faction: Faction) -> Color32 {
    match faction {
        Faction::Player => Color32::from_rgb(45, 90, 120),
        Faction::Ally => Color32::from_rgb(0, 120, 90),
        Faction::Enemy => Color32::from_rgb(120, 45, 0),
    }
}

pub fn vessel_normal_circle_color(faction: Faction) -> Color32 {
    match faction {
        Faction::Player => Color32::from_rgb(30, 60, 80),
        Faction::Ally => Color32::from_rgb(0, 80, 60),
        Faction::Enemy => Color32::from_rgb(80, 30, 0),
    }
}

fn root_entities(view: &View) -> HashSet<Entity> {
    view.model
        .entities(vec![ComponentType::OrbitableComponent])
        .into_iter()
        .filter(|entity| view.model.parent(*entity).is_none())
        .collect()
}

fn order_by_altitude(view: &View, mut entities: Vec<Entity>) -> Vec<Entity> {
    entities.sort_by(|a, b| {
        let altitude_a = view.model.position(*a).magnitude();
        let altitude_b = view.model.position(*b).magnitude();
        altitude_a.partial_cmp(&altitude_b).unwrap()
    });
    entities
}

fn child_vessel_entities(view: &View, parent_entity: Entity) -> Vec<Entity> {
    let entities: Vec<Entity> = view
        .model
        .entities(vec![ComponentType::VesselComponent])
        .into_iter()
        .filter(|entity| view.model.try_vessel_component(*entity).is_some())
        .filter(|entity| !view.model.vessel_component(*entity).is_ghost())
        .filter(|entity| {
            view.model
                .parent(*entity)
                .is_some_and(|parent| parent == parent_entity)
        })
        .collect();
    order_by_altitude(view, entities)
}

pub fn child_orbitable_entities(view: &View, parent_entity: Entity) -> Vec<Entity> {
    let entities: Vec<Entity> = view
        .model
        .entities(vec![ComponentType::OrbitableComponent])
        .into_iter()
        .filter(|entity| view.model.try_orbitable_component(*entity).is_some())
        .filter(|entity| {
            view.model
                .parent(*entity)
                .is_some_and(|parent| parent == parent_entity)
        })
        .collect();
    order_by_altitude(view, entities)
}

fn render_orbitable(view: &View, ui: &mut Ui, entity: Entity) {
    let orbitable_component = view.model.orbitable_component(entity);
    let texture = orbitable_texture(orbitable_component.type_());
    let name = view.model.name_component(entity).name();
    ui.add_space(-7.0);
    let button = CustomCircularImageButton::new(view, texture, 24.0)
        .with_padding(4.0)
        .with_margin(2.0)
        .with_normal_color(Color32::from_rgb(60, 60, 60))
        .with_hover_color(Color32::from_rgb(90, 90, 90));
    if ui.add(button).clicked() {
        view.add_view_event(ViewEvent::SetSelected(Selected::Orbitable(entity)));
    }
    ui.add_space(-5.0);
    ui.label(RichText::new(name).size(14.0).strong());
}

fn render_vessel(view: &View, ui: &mut Ui, entity: Entity) {
    let vessel_component = view.model.vessel_component(entity);
    let texture = vessel_texture(vessel_component);
    let name = view.model.name_component(entity).name();
    let faction = vessel_component.faction();
    ui.add_space(-7.0);
    let button = CustomCircularImageButton::new(view, texture, 24.0)
        .with_padding(4.0)
        .with_margin(2.0)
        .with_normal_color(vessel_normal_circle_color(faction))
        .with_hover_color(vessel_hover_circle_color(faction));
    if ui.add(button).clicked() {
        view.add_view_event(ViewEvent::SetSelected(Selected::Vessel(entity)));
    }
    ui.add_space(-5.0);
    ui.label(RichText::new(name).size(14.0).strong());
}

fn render_entity(view: &View, ui: &mut Ui, entity: Entity, levels: &[bool]) {
    ui.horizontal(|ui| {
        if levels.is_empty() {
            ui.add_space(7.0);
        }
        ui.add_space(4.0);
        for i in 0..levels.len() {
            let level = levels[i];
            let is_last_level = i == levels.len() - 1;
            if level && is_last_level {
                ui.add(CustomImage::new(view, "explorer-intersection", 24.0));
            } else if level && !is_last_level {
                ui.add(CustomImage::new(view, "explorer-straight", 24.0));
            } else if is_last_level {
                ui.add(CustomImage::new(view, "explorer-corner", 24.0));
            } else {
                ui.add_space(24.0);
            }
        }
        if view.model.try_vessel_component(entity).is_some() {
            render_vessel(view, ui, entity);
        }
        if view.model.try_orbitable_component(entity).is_some() {
            render_orbitable(view, ui, entity);
        }
    });

    ui.add_space(-3.0);

    if view.model.try_orbitable_component(entity).is_some() {
        let mut to_render = vec![];
        for other_entity in child_vessel_entities(view, entity) {
            to_render.push(other_entity);
        }
        for other_entity in child_orbitable_entities(view, entity) {
            to_render.push(other_entity);
        }

        for i in 0..to_render.len() {
            let mut levels = levels.to_vec();
            let level = i != to_render.len() - 1;
            levels.push(level);
            render_entity(view, ui, to_render[i], &levels);
        }
    }
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update explorer");

    if !matches!(view.selected, Selected::None) {
        return;
    }

    Window::new("Explorer")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(&view.context.clone(), |ui| {
            draw_title(ui, "Overview");

            for entity in root_entities(view) {
                render_entity(view, ui, entity, &[]);
            }
        });
}
