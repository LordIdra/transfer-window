use eframe::egui::{Align2, Context, Ui, Window};
use nalgebra_glm::vec2;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::Scene, styles};

use super::widgets::custom_image_button::CustomCircularImageButton;

fn draw_focus(view: &mut Scene, ui: &mut Ui, context: &Context, entity: Entity) {
    let button = CustomCircularImageButton::new(view.renderers.get_screen_texture_renderer("focus"), context.screen_rect(), 30.0)
        .with_padding(3.0);
    if ui.add(button).on_hover_text("Focus").clicked() {
        view.camera.reset_panning();
        view.camera.set_focus(Some(entity));
        view.right_click_menu = None;
    }
}

fn draw_set_target(view: &mut Scene, model: &Model, ui: &mut Ui, context: &Context, right_clicked: Entity, selected: Entity, events: &mut Vec<Event>) {
    let is_already_target = model.target(selected) == Some(right_clicked);
    if is_already_target {
        let button = CustomCircularImageButton::new(view.renderers.get_screen_texture_renderer("unset-target"), context.screen_rect(), 30.0)
            .with_padding(4.0);
        if ui.add(button).on_hover_text("Unset target").clicked() {
            events.push(Event::SetTarget { 
                entity: selected, 
                target: None,
            });
            view.right_click_menu = None;
        }
    } else {
        let enabled = selected != right_clicked;
        let button = CustomCircularImageButton::new(view.renderers.get_screen_texture_renderer("set-target"), context.screen_rect(), 30.0)
            .with_enabled(enabled)
            .with_padding(4.0);
        if ui.add_enabled(enabled, button)
                .on_hover_text("Set target")
                .clicked() {
            events.push(Event::SetTarget { 
                entity: selected, 
                target: Some(right_clicked),
            });
            view.right_click_menu = None;
        }
    }
}

fn draw(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, right_clicked: Entity) {
    let world_position = model.absolute_position(right_clicked) + vec2(50.0, 0.0);
    let window_position = view.camera.world_space_to_window_space(model, world_position, context.screen_rect());
    Window::new("Right click menu")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_BOTTOM, window_position.to_vec2())
        .show(context, |ui| {
            ui.horizontal(|ui| {
                styles::SelectedMenuButton::apply(ui);
                ui.set_height(30.0);
                draw_focus(view, ui, context, right_clicked);
                if let Some(selected) = view.selected.entity(model) {
                    if model.try_vessel_component(selected).is_some() {
                        draw_set_target(view, model, ui, context, right_clicked, selected, events);
                    }
                }
            });
        });
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, is_mouse_over_any_icon: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update right click menu");

    let is_mouse_over_ui_element = view.pointer_over_ui_last_frame || is_mouse_over_any_icon;
    if !is_mouse_over_ui_element {
        let pointer = context.input(|input| {
            input.pointer.clone()
        });
        if view.right_click_menu.is_some() && pointer.primary_clicked() {
            view.right_click_menu = None;
        }
    }

    if let Some(entity) = view.right_click_menu {
        draw(view, model, context, events, entity);
    }
}