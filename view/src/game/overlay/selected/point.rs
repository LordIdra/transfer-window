use eframe::{egui::{Align2, Context, RichText, Ui, Window}, epaint};
use transfer_window_model::{components::{path_component::orbit::Orbit, vessel_component::timeline::{enable_guidance::EnableGuidanceEvent, start_burn::StartBurnEvent}}, storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::{overlay::widgets::custom_image_button::CustomCircularImageButton, selected::{util::BurnState, Selected}, util::format_time, Scene}, styles};

use self::weapons::draw_weapons;

mod weapons;

fn draw_previous(time: f64, period: f64, orbit: &Orbit, ui: &mut Ui, view: &mut Scene, context: &Context, entity: Entity) {
    let time = time - period;
    let enabled = time > orbit.current_point().time();
    let button = CustomCircularImageButton::new(view, "previous-orbit", context.screen_rect(), 36.0)
        .with_enabled(enabled)
        .with_padding(10.0);
    if ui.add_enabled(enabled, button).on_hover_text("Previous orbit").clicked() {
        view.selected = Selected::Point { entity, time };
    }
}

fn draw_next(time: f64, period: f64, orbit: &Orbit, ui: &mut Ui, view: &mut Scene, context: &Context, entity: Entity) {
    let time = time + period;
    let enabled = time < orbit.end_point().time();
    let button = CustomCircularImageButton::new(view, "next-orbit", context.screen_rect(), 36.0)
        .with_enabled(enabled)
        .with_padding(10.0);
    if ui.add_enabled(enabled, button).on_hover_text("Next orbit").clicked() {
        view.selected = Selected::Point { entity, time };
    }
}

fn draw_orbits(time: f64, period: f64, orbit: &Orbit, ui: &mut Ui) {
    let orbits = ((time - orbit.current_point().time()) / period) as usize;
    if orbits != 0 {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Orbits:").strong().monospace());
            ui.label(orbits.to_string());
        });
    }
}

fn draw_vessel(model: &Model, entity: Entity, ui: &mut Ui, events: &mut Vec<Event>, time: f64, view: &mut Scene, context: &Context) {
    ui.label(RichText::new("Orbit").size(20.0).monospace().strong());
    let text = format!("T-{}", format_time(time - model.time()));
    ui.label(RichText::new(text).weak());

    let orbit = model.path_component(entity)
        .future_segment_at_time(time)
        .as_orbit()
        .expect("Segment at requested time is not orbit");

    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);
        ui.set_height(36.0);

        if let Some(period) = orbit.period() {
            draw_previous(time, period, orbit, ui, view, context, entity);
            draw_next(time, period, orbit, ui, view, context, entity);
        }

        let enabled = model.can_warp_to(time);
        let button = CustomCircularImageButton::new(view, "warp-here", context.screen_rect(), 36.0)
            .with_enabled(enabled)
            .with_padding(8.0);
        if ui.add_enabled(enabled, button).on_hover_text("Warp here").clicked() {
            events.push(Event::StartWarp { end_time: time });
        }

        if StartBurnEvent::can_create_ever(model, entity) {
            let enabled = StartBurnEvent::can_create(model, entity, time);
            let button = CustomCircularImageButton::new(view, "create-burn", context.screen_rect(), 36.0)
                .with_enabled(enabled)
                .with_padding(5.0);
            if ui.add_enabled(enabled, button).on_hover_text("Create burn").clicked() {
                events.push(Event::CreateBurn { entity, time });
                view.selected = Selected::Burn { entity, time, state: BurnState::Selected }
            }
        }

        if EnableGuidanceEvent::can_create_ever(model, entity) {
            let enabled = EnableGuidanceEvent::can_create(model, entity, time);
            let button = CustomCircularImageButton::new(view, "enable-guidance", context.screen_rect(), 36.0)
                .with_enabled(enabled)
                .with_padding(5.0);
            if ui.add_enabled(enabled, button).on_hover_text("Enable guidance").clicked() {
                events.push(Event::CreateGuidance { entity, time });
                view.selected = Selected::EnableGuidance { entity, time }
            }
        }
    });
    
    if let Some(period) = orbit.period() {
        draw_orbits(time, period, orbit, ui);
    }
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
