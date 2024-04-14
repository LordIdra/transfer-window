use eframe::{egui::{Align2, Button, Context, Ui, Window}, epaint};
use transfer_window_model::{components::trajectory_component::orbit::Orbit, storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::{underlay::selected::{burn::BurnState, segment_point::SegmentPointState, Selected}, util::format_time, Scene}};

fn draw_next(time: f64, period: f64, segment: &Orbit, ui: &mut Ui, view: &mut Scene, entity: Entity, state: SegmentPointState) {
    let time = time - period;
    let button = Button::new("Previous orbit");
    let enabled = time > segment.get_current_point().get_time();
    if ui.add_enabled(enabled, button).clicked() {
        view.selected = Selected::Point { entity, time, state };
    }
}

fn draw_previous(time: f64, period: f64, segment: &Orbit, ui: &mut Ui, view: &mut Scene, entity: Entity, state: SegmentPointState) {
    let time = time + period;
    let button = Button::new("Next orbit");
    let enabled = time < segment.get_end_point().get_time();
    if ui.add_enabled(enabled, button).clicked() {
        view.selected = Selected::Point { entity, time, state };
    }
}

fn draw_orbits(time: f64, model: &Model, period: f64, ui: &mut Ui) {
    let orbits = ((time - model.get_time()) / period) as usize;
    if orbits != 0 {
        ui.label("Orbits: ".to_string() + orbits.to_string().as_str());
    }
}

fn draw_vessel(model: &Model, entity: Entity, ui: &mut Ui, events: &mut Vec<Event>, time: f64, view: &mut Scene, state: SegmentPointState) {
    if ui.button("Create burn").clicked() {
        events.push(Event::CreateBurn { entity, time });
        view.selected = Selected::Burn { entity, time, state: BurnState::Selected }
    }

    let segment = model.get_trajectory_component(entity).get_first_segment_at_time(time).as_orbit();
    if let Some(period) = segment.get_period() {
        draw_next(time, period, segment, ui, view, entity, state.clone());
        draw_previous(time, period, segment, ui, view, entity, state);
        draw_orbits(time, model, period, ui);
    }
}

pub fn update(view: &mut Scene, context: &Context, model: &Model, events: &mut Vec<Event>) {
    let Selected::Point { entity, time, state } = view.selected.clone() else {
        return;
    };

    if !state.is_selected() {
        return;
    }

    Window::new("Selected point")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(30.0, 30.0))
        .show(context, |ui| {
            ui.label(model.get_name_component(entity).get_name());
            ui.label("T-".to_string() + format_time(time - model.get_time()).as_str());

            if ui.button("Warp here").clicked() {
                events.push(Event::StartWarp { end_time: time });
            }

            if model.try_get_vessel_component(entity).is_some() {
                draw_vessel(model, entity, ui, events, time, view, state);
            }
        });
}
