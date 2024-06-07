use eframe::{egui::{Align2, Context, Grid, Ui, Window}, epaint};
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::{overlay::widgets::{buttons::{draw_create_burn, draw_enable_guidance, draw_next, draw_previous, draw_warp_to}, labels::{draw_altitude, draw_orbits, draw_speed, draw_time_until, draw_title}}, selected::{util::BurnState, Selected}, Scene}, styles};

use self::weapons::draw_weapons;

mod weapons;

fn draw_vessel(model: &Model, entity: Entity, ui: &mut Ui, events: &mut Vec<Event>, time: f64, view: &mut Scene, context: &Context) {
    draw_title(ui, "Orbit");
    draw_time_until(model, ui, time);

    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);

        if let Some(time) = draw_previous(view, model, context, ui, time, entity) {
            view.selected = Selected::Point { entity, time };
        }

        if let Some(time) = draw_next(view, model, context, ui, time, entity) {
            view.selected = Selected::Point { entity, time };
        }

        if draw_warp_to(view, model, context, ui, time) {
            events.push(Event::StartWarp { end_time: time });
        }

        if draw_create_burn(view, model, context, ui, entity, time) {
            events.push(Event::CreateBurn { entity, time });
            view.selected = Selected::Burn { entity, time, state: BurnState::Selected }
        }

        if draw_enable_guidance(view, model, context, ui, entity, time) {
            events.push(Event::CreateGuidance { entity, time });
            view.selected = Selected::EnableGuidance { entity, time }
        }
    });
    
    Grid::new("Selected point info").show(ui, |ui| {
        draw_altitude(model, ui, entity, time);
        draw_speed(model, ui, entity, time);
        draw_orbits(model, ui, entity, time);
    });
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update point");
    let Selected::Point { entity, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected point")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            draw_vessel(model, entity, ui, events, time, view, context);
        });

    let weapon_slots = model.vessel_component(entity).slots().weapon_slots();
    if !weapon_slots.is_empty() {
        Window::new("Weapons")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::CENTER_BOTTOM, epaint::vec2(0.0, 0.0))
            .show(context, |ui| {
                draw_weapons(view, model, ui, context, entity, &weapon_slots, time, events);
            });
    }
}
