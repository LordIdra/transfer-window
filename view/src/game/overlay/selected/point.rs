use eframe::{egui::{Align2, Button, Context, Ui, Window}, epaint};
use transfer_window_model::{components::path_component::orbit::Orbit, storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::{underlay::selected::{util::BurnState, Selected}, util::format_time, Scene}};

use self::weapons::draw_weapons;

mod weapons;

fn draw_next(time: f64, period: f64, orbit: &Orbit, ui: &mut Ui, view: &mut Scene, entity: Entity) {
    let time = time - period;
    let button = Button::new("Previous orbit");
    let enabled = time > orbit.current_point().time();
    if ui.add_enabled(enabled, button).clicked() {
        view.selected = Selected::Point { entity, time };
    }
}

fn draw_previous(time: f64, period: f64, orbit: &Orbit, ui: &mut Ui, view: &mut Scene, entity: Entity) {
    let time = time + period;
    let button = Button::new("Next orbit");
    let enabled = time < orbit.end_point().time();
    if ui.add_enabled(enabled, button).clicked() {
        view.selected = Selected::Point { entity, time };
    }
}

fn draw_orbits(time: f64, period: f64, orbit: &Orbit, ui: &mut Ui) {
    let orbits = ((time - orbit.current_point().time()) / period) as usize;
    if orbits != 0 {
        ui.label("Orbits: ".to_string() + orbits.to_string().as_str());
    }
}

fn draw_vessel(model: &Model, entity: Entity, ui: &mut Ui, events: &mut Vec<Event>, time: f64, view: &mut Scene) {
    let has_burn_before_requested_time = if let Some(final_burn) = model.path_component(entity).final_burn() {
        time < final_burn.start_point().time()
    } else {
        false
    };
    let vessel_component = model.vessel_component(entity);
    let can_create_burn = vessel_component.slots().engine().is_some() && !has_burn_before_requested_time;

    let create_burn_button = Button::new("Create burn");
    if ui.add_enabled(can_create_burn, create_burn_button).clicked() {
        events.push(Event::CreateBurn { entity, time });
        view.selected = Selected::Burn { entity, time, state: BurnState::Selected }
    }

    if vessel_component.class().is_torpedo() {
        let button = Button::new("Enable guidance");
        if ui.add_enabled(model.can_torpedo_enable_guidance(entity), button).clicked() {
            events.push(Event::EnableTorpedoGuidance { entity, time });
        }
    }

    let orbit = model.path_component(entity)
        .future_segment_at_time(time)
        .as_orbit()
        .expect("Segment at requested time is not orbit");
    if let Some(period) = orbit.period() {
        draw_next(time, period, orbit, ui, view, entity);
        draw_previous(time, period, orbit, ui, view, entity);
        draw_orbits(time, period, orbit, ui);
    }
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::Point { entity, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected point")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label(model.name_component(entity).name());
            ui.label("T-".to_string() + format_time(time - model.time()).as_str());

            if ui.button("Warp here").clicked() {
                events.push(Event::StartWarp { end_time: time });
            }

            if model.try_vessel_component(entity).is_some() {
                draw_vessel(model, entity, ui, events, time, view);
            }
        });

    let vessel_component = model.vessel_component(entity);
    let weapon_slots = vessel_component.slots().weapon_slots();
    if !weapon_slots.is_empty() {
        Window::new("Weapons")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::CENTER_BOTTOM, epaint::vec2(0.0, 0.0))
            .show(context, |ui| {
                draw_weapons(view, ui, vessel_component, entity, &weapon_slots, time, events);
            });
    }
}
