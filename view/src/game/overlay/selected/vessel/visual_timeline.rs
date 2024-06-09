#![allow(clippy::match_same_arms)]

use eframe::egui::{Color32, Frame, RichText, Ui};
use transfer_window_model::{api::encounters::EncounterType, components::vessel_component::timeline::TimelineEvent, storage::entity_allocator::Entity};

use crate::game::{events::ViewEvent, overlay::widgets::custom_image_button::CustomCircularImageButton, selected::{util::BurnState, Selected}, util::{format_distance, format_time, ApproachType, ApsisType}, View};

enum VisualTimelineEvent {
    TimelineEvent(TimelineEvent),
    Apsis { type_: ApsisType, time: f64, distance: f64 },
    Approach { type_: ApproachType, target: Entity, time: f64, distance: f64 },
    Encounter { type_: EncounterType, time: f64, from: Entity, to: Entity },
    Point { time: f64 },
}

impl VisualTimelineEvent {
    pub fn time(&self) -> f64 {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => event.time(),
            VisualTimelineEvent::Apsis { time, .. } 
                | VisualTimelineEvent::Approach { time, .. } 
                | VisualTimelineEvent::Point { time }
                | VisualTimelineEvent::Encounter { time, .. } => *time,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => match event {
                TimelineEvent::Intercept(_) => "intercept",
                TimelineEvent::FireTorpedo(_) => "torpedo",
                TimelineEvent::Burn(_) => "burn",
                TimelineEvent::EnableGuidance(_) => "enable-guidance",
            }
            VisualTimelineEvent::Apsis { type_, .. } => match type_ {
                ApsisType::Periapsis => "periapsis",
                ApsisType::Apoapsis => "apoapsis",
            }
            VisualTimelineEvent::Approach { type_, .. } => match type_ {
                ApproachType::First => "closest-approach-1",
                ApproachType::Second => "closest-approach-2",
            }
            VisualTimelineEvent::Encounter { type_, .. } => match type_ {
                EncounterType::Entrance => "encounter-entrance",
                EncounterType::Exit => "encounter-exit",
            }
            VisualTimelineEvent::Point { .. } => "circle",
        }
    }

    pub fn padding(&self) -> f32 {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => match event {
                TimelineEvent::Intercept(_) => 3.0,
                TimelineEvent::FireTorpedo(_) => 0.0,
                TimelineEvent::Burn(_) => 0.0,
                TimelineEvent::EnableGuidance(_) => 0.0,
            }
            VisualTimelineEvent::Apsis { .. } => 3.0,
            VisualTimelineEvent::Approach { .. } => 3.0,
            VisualTimelineEvent::Encounter { .. } => 3.0,
            VisualTimelineEvent::Point { .. } => 5.0,
        }
    }

    pub fn name(&self, view: &View) -> String {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => match event {
                TimelineEvent::Intercept(_) => "Intercept".to_string(),
                TimelineEvent::FireTorpedo(_) => "Torpedo Launch".to_string(),
                TimelineEvent::Burn(_) => "Burn".to_string(),
                TimelineEvent::EnableGuidance(_) => "Guidance Enabled".to_string(),
            }
            VisualTimelineEvent::Apsis { type_, distance, .. } => match type_ {
                ApsisType::Periapsis => format!("Periapsis - {}", format_distance(*distance)),
                ApsisType::Apoapsis => format!("Apoapsis - {}", format_distance(*distance)),
            }
            VisualTimelineEvent::Approach { distance, .. } => format!("Approach - {}", format_distance(*distance)),
            VisualTimelineEvent::Encounter { type_, from, to, .. } => match type_ {
                EncounterType::Entrance => format!("{} Entrance", view.model.name_component(*to).name()),
                EncounterType::Exit => format!("{} Exit", view.model.name_component(*from).name()),
            }
            VisualTimelineEvent::Point { .. } => "Selected point".to_string(),
        }
    }

    pub fn selected(&self, entity: Entity) -> Option<Selected> {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => Some(match event {
                TimelineEvent::Intercept(intercept) => Selected::Intercept { entity, time: intercept.time() },
                TimelineEvent::FireTorpedo(fire_torpedo) => Selected::FireTorpedo { entity, time: fire_torpedo.time(), state: BurnState::Selected },
                TimelineEvent::Burn(burn) => Selected::Burn { entity, time: burn.time(), state: BurnState::Selected },
                TimelineEvent::EnableGuidance(enable_guidance) => Selected::EnableGuidance { entity, time: enable_guidance.time() },
            }),
            VisualTimelineEvent::Apsis { type_, time, distance: _ } => Some(Selected::Apsis { type_: *type_, entity, time: *time }),
            VisualTimelineEvent::Approach { type_, target, time, distance: _ } => Some(Selected::Approach { type_: *type_, entity, target: *target, time: *time }),
            VisualTimelineEvent::Encounter { type_, time, from, to } => Some(Selected::Encounter { type_: *type_, entity, time: *time, from: *from, to: *to }),
            VisualTimelineEvent::Point { .. } => None,
        }
    }

    /// We try to avoid using times to compare events here because times are sometimes subject to small errors
    /// depending on the numerical methods used to calculate them
    /// As a result this block of code is horrible :sob:
    pub fn is_selected(&self, view: &View, other_entity: Entity) -> bool {
        if view.selected.entity(&view.model).is_some_and(|entity| entity != other_entity) {
            return false;
        };

        match view.selected {
            Selected::None => false,
            Selected::Orbitable(_) => false,
            Selected::Vessel(_) => false,
            Selected::Point { .. } => match self {
                VisualTimelineEvent::Point { .. } => true,
                _ => false
            },
            Selected::Apsis { type_, .. } => {
                match self {
                    VisualTimelineEvent::Apsis { type_: other_type, .. } => type_ == *other_type,
                    _ => false,
                }
            }
            Selected::Approach { type_, .. } => {
                match self {
                    VisualTimelineEvent::Approach { type_: other_type, .. } => type_ == *other_type,
                    _ => false,
                }
            }
            Selected::Encounter { type_, from, to, .. } => {
                match self {
                    VisualTimelineEvent::Encounter { type_: other_type, from: other_from, to: other_to, .. } => type_ == *other_type && from == *other_from && to == *other_to,
                    _ => false,
                }
            }
            Selected::Intercept { .. } => match self {
                VisualTimelineEvent::TimelineEvent(event) => event.is_intercept(), // only 1 intercept at a time is possible (in theory)
                _ => false,
            }
            Selected::Burn { time, .. } => match self {
                VisualTimelineEvent::TimelineEvent(event) => {
                    match event {
                        TimelineEvent::Burn(event) => event.time() == time,
                        _ => false
                    }
                },
                _ => false,
            }
            Selected::FireTorpedo { time, .. } => match self {
                VisualTimelineEvent::TimelineEvent(event) => {
                    match event {
                        TimelineEvent::FireTorpedo(event) => event.time() == time,
                        _ => false
                    }
                },
                _ => false,
            }
            Selected::EnableGuidance { time, .. } => match self {
                VisualTimelineEvent::TimelineEvent(event) => {
                    match event {
                        TimelineEvent::EnableGuidance(event) => event.time() == time,
                        _ => false
                    }
                },
                _ => false,
            }
        }
    }
}

fn generate_timeline_events(view: &View, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    for event in view.model.vessel_component(entity).timeline().events() {
        events.push(VisualTimelineEvent::TimelineEvent(event.clone()));
    }
}

fn generate_apoapsis_periapsis(view: &View, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    for orbit in view.model.path_component(entity).future_orbits() {
        if let Some(time) = orbit.next_periapsis_time() {
            let distance = view.model.position_at_time(entity, time).magnitude();
            events.push(VisualTimelineEvent::Apsis { type_: ApsisType::Periapsis, time, distance });
        }

        if let Some(time) = orbit.next_apoapsis_time() {
            let distance = view.model.position_at_time(entity, time).magnitude();
            events.push(VisualTimelineEvent::Apsis { type_: ApsisType::Apoapsis, time, distance });
        }
    }
}

fn generate_closest_approaches(view: &View, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    let Some(target) = view.model.vessel_component(entity).target() else { 
        return;
    };

    let (approach_1_time, approach_2_time) = view.model.find_next_two_closest_approaches(entity, target);

    if let Some(time) = approach_1_time {
        let distance = view.model.distance_at_time(entity, target, time);
        events.push(VisualTimelineEvent::Approach { type_: ApproachType::First, target, time, distance });
    }

    if let Some(time) = approach_2_time {
        let distance = view.model.distance_at_time(entity, target, time);
        events.push(VisualTimelineEvent::Approach { type_: ApproachType::Second, target, time, distance });
    }
}

fn generate_encounters(view: &View, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    for encounter in view.model.future_encounters(entity) {
        events.push(VisualTimelineEvent::Encounter { type_: encounter.encounter_type(), time: encounter.time(), from: encounter.from(), to: encounter.to() });
    }
}

fn advance_cursor_to(ui: &mut Ui, x: f32) {
    let width = x - ui.cursor().left();
    let mut rect = ui.cursor();
    rect.set_width(width);
    rect.set_height(0.0);
    ui.advance_cursor_after_rect(rect);
}

pub fn draw(view: &View, ui: &mut Ui, entity: Entity, center_time: f64, draw_center_time_point: bool) {
    let mut events = vec![];

    generate_timeline_events(view, entity, &mut events);
    generate_apoapsis_periapsis(view, entity, &mut events);
    generate_closest_approaches(view, entity, &mut events);
    generate_encounters(view, entity, &mut events);

    if draw_center_time_point {
        events.push(VisualTimelineEvent::Point { time: center_time });
    }

    events.sort_by(|a, b| a.time().total_cmp(&b.time()));

    if let Some(last_event) = view.model.vessel_component(entity).timeline().last_event() {
        if last_event.is_intercept() {
            events.retain(|event| event.time() <= last_event.time());
        }
    }

    for event in events {
        draw_event(view, ui, event, entity, center_time);
    }
}

fn draw_event(view: &View, ui: &mut Ui, event: VisualTimelineEvent, entity: Entity, center_time: f64) {
    let time_until = (event.time().ceil() - center_time).floor();
    let fill = if event.is_selected(view, entity) {
        Color32::from_rgba_premultiplied(35, 35, 60, 200)
    } else if time_until.is_sign_positive() {
        Color32::from_rgba_premultiplied(30, 30, 30, 200)
    } else {
        Color32::from_rgba_premultiplied(20, 20, 20, 200)
    };

    Frame::default()
            .fill(fill)
            .show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.style_mut().visuals.panel_fill = Color32::RED;
            let image = CustomCircularImageButton::new(view, event.icon(), 20.0)
                .with_padding(event.padding());
            if ui.add(image).clicked() {
                if let Some(selected) = event.selected(entity) {
                    view.add_view_event(ViewEvent::SetSelected(selected))
                }
            }

            let time_text = if event.is_selected(view, entity) {
                "now".to_string()
            } else if time_until.is_sign_positive() {
                format!("T-{}", format_time(time_until))
            } else {
                format!("T+{}", format_time(-time_until))
            };
            ui.label(RichText::new(time_text).weak().size(12.0));

            advance_cursor_to(ui, 150.0);

            ui.label(RichText::new(event.name(view)));
            advance_cursor_to(ui, 320.0);
            ui.end_row();
        });
    });
}