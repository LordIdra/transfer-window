use eframe::egui::{Align2, Ui, Window};
use nalgebra_glm::vec2;
use transfer_window_model::{components::vessel_component::Faction, storage::entity_allocator::Entity};

use crate::{game::{events::{ModelEvent, ViewEvent}, View}, styles};

use super::widgets::{buttons, custom_image_button::CustomCircularImageButton};

fn draw_focus(view: &View, ui: &mut Ui, entity: Entity) {
    if buttons::draw_focus(view, ui) {
        view.add_view_event(ViewEvent::ResetCameraPanning);
        view.add_view_event(ViewEvent::SetCameraFocus(entity));
        view.add_view_event(ViewEvent::HideRightClickMenu);
    }
}

fn draw_set_target(view: &View, ui: &mut Ui, right_clicked: Entity, selected: Entity) {
    let is_already_target = view.model.target(selected) == Some(right_clicked);
    if is_already_target {
        let button = CustomCircularImageButton::new(view, "unset-target", 30.0)
            .with_padding(4.0);
        if ui.add(button).on_hover_text("Unset target").clicked() {
            view.add_model_event(ModelEvent::SetTarget { 
                entity: selected, 
                target: None,
            });
            view.add_view_event(ViewEvent::HideRightClickMenu);
        }
    } else {
        let enabled = selected != right_clicked;
        let button = CustomCircularImageButton::new(view, "set-target", 30.0)
            .with_enabled(enabled)
            .with_padding(4.0);
        if ui.add_enabled(enabled, button)
                .on_hover_text("Set target")
                .clicked() {
            view.add_model_event(ModelEvent::SetTarget { 
            entity: selected, 
            target: Some(right_clicked),
        });
        view.add_view_event(ViewEvent::HideRightClickMenu);
        }
    }
}

fn draw(view: &View, right_clicked: Entity) {
    let world_position = view.model.absolute_position(right_clicked) + vec2(50.0, 0.0);
    let window_position = view.world_space_to_window_space(world_position);
    Window::new("Right click menu")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_BOTTOM, window_position.to_vec2())
        .show(&view.context.clone(), |ui| {
            ui.horizontal(|ui| {
                styles::SelectedMenuButton::apply(ui);
                ui.set_height(30.0);
                draw_focus(view, ui, right_clicked);
                if let Some(selected) = view.selected.entity(&view.model) {
                    if let Some(vessel_component) = view.model.try_vessel_component(selected) {
                        if Faction::Player.can_control(vessel_component.faction()) {
                            draw_set_target(view, ui, right_clicked, selected);
                        }
                    }
                }
            });
        });
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update right click menu");

    let is_mouse_over_ui_element = view.pointer_over_ui || view.pointer_over_icon;
    if !is_mouse_over_ui_element {
        let pointer = view.context.input(|input| {
            input.pointer.clone()
        });
        if view.right_click_menu.is_some() && pointer.primary_clicked() {
            view.add_view_event(ViewEvent::HideRightClickMenu);
        }
    }

    if let Some(entity) = view.right_click_menu {
        draw(view, entity);
    }
}