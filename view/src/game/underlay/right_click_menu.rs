use eframe::egui::{Align2, Button, Context, Ui, Window};
use nalgebra_glm::vec2;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::Scene};

fn draw_focus(view: &mut Scene, ui: &mut Ui, entity: Entity) {
    if ui.button("Focus").clicked() {
        view.camera.reset_panning();
        view.camera.set_focus(Some(entity));
    }
}

fn draw_set_target(model: &Model, ui: &mut Ui, right_clicked: Entity, selected: Entity, events: &mut Vec<Event>) {
    let is_already_target = model.target(selected) == Some(right_clicked);
    if is_already_target {
        let button = Button::new("Unset target");
        if ui.add(button).clicked() {
            events.push(Event::SetTarget { 
                entity: selected, 
                target: None,
            });
        }
    } else {
        let can_target = selected != right_clicked;
        let button = Button::new("Set target");
        if ui.add_enabled(can_target, button).clicked() {
            events.push(Event::SetTarget { 
                entity: selected, 
                target: Some(right_clicked),
            });
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
            draw_focus(view, ui, right_clicked);
            if let Some(selected) = view.selected.entity(model) {
                if model.try_vessel_component(selected).is_some() {
                    draw_set_target(model, ui, right_clicked, selected, events);
                }
            }
        });
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, is_mouse_over_any_icon: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update right click menu");

    let is_mouse_over_ui_element = context.is_pointer_over_area() || is_mouse_over_any_icon;
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