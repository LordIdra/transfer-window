use eframe::{egui::{Align2, Color32, RichText, Ui, Window}, epaint};
use transfer_window_model::{components::{vessel_component::Faction, ComponentType}, storage::entity_allocator::Entity};

use crate::game::{events::ViewEvent, selected::Selected, util::{orbitable_texture, vessel_texture}};

use super::{widgets::{custom_image::CustomImage, custom_image_button::CustomCircularImageButton, labels::draw_title}, View};

fn vessel_hover_circle_color(faction: Faction) -> Color32 {
    match faction {
        Faction::Player => Color32::from_rgb(45, 90, 120),
        Faction::Ally => Color32::from_rgb(0, 120, 90),
        Faction::Enemy => Color32::from_rgb(120, 45, 0),
    }
}

fn vessel_normal_circle_color(faction: Faction) -> Color32 {
    match faction {
        Faction::Player => Color32::from_rgb(30, 60, 80),
        Faction::Ally => Color32::from_rgb(0, 80, 60),
        Faction::Enemy => Color32::from_rgb(80, 30, 0),
    }
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
    let texture = vessel_texture(vessel_component.class());
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
                // if levels.len() > 1 {
                //     ui.add_space(-7.0);
                // }
                ui.add(CustomImage::new(view, "explorer-intersection", 24.0));
            } else if level && !is_last_level {
                // if levels.len() > 1 {
                //     ui.add_space(-7.0);
                // }
                ui.add(CustomImage::new(view, "explorer-straight", 24.0));
            } else if is_last_level {
                // if levels.len() > 1 {
                //     ui.add_space(-7.0);
                // }
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
        for other_entity in view.model.entities(vec![ComponentType::VesselComponent]) {
            if !view.model.vessel_component(other_entity).is_ghost() && view.model.parent(other_entity).is_some_and(|parent| parent == entity) {
                to_render.push(other_entity);
            }
        }
        for other_entity in view.model.entities(vec![ComponentType::OrbitableComponent]) {
            if view.model.parent(other_entity).is_some_and(|parent| parent == entity) {
                to_render.push(other_entity);
            }
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
        
        for entity in view.model.entities(vec![ComponentType::OrbitableComponent]) {
            if view.model.parent(entity).is_none() {
                render_entity(view, ui, entity, &[]);
            }
        }
    });
}