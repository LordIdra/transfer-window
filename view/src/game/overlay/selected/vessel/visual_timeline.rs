#![allow(clippy::match_same_arms)]
use eframe::egui::{RichText, Ui};
use transfer_window_model::{api::encounters::EncounterType, components::vessel_component::timeline::TimelineEvent, storage::entity_allocator::Entity};

use crate::game::{overlay::widgets::custom_image::CustomImage, util::{format_distance, format_time, ApproachType, ApsisType}, View};

enum VisualTimelineEvent {
    TimelineEvent(TimelineEvent),
    Apsis { type_: ApsisType, time: f64, distance: f64 },
    Approach { type_: ApproachType, time: f64, distance: f64 },
    Encounter { type_: EncounterType, time: f64, other_entity: Entity },
}

impl VisualTimelineEvent {
    pub fn time(&self) -> f64 {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => event.time(),
            VisualTimelineEvent::Apsis { time, .. } 
                | VisualTimelineEvent::Approach { time, .. } 
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
            VisualTimelineEvent::Approach { distance, .. } => format!("Closest Approach - {}", format_distance(*distance)),
            VisualTimelineEvent::Encounter { type_, other_entity, .. } => match type_ {
                EncounterType::Entrance => format!("{} Entrance", view.model.name_component(*other_entity).name()),
                EncounterType::Exit => format!("{} Exit", view.model.name_component(*other_entity).name()),
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
        events.push(VisualTimelineEvent::Approach { type_: ApproachType::First, time, distance });
    }

    if let Some(time) = approach_2_time {
        let distance = view.model.distance_at_time(entity, target, time);
        events.push(VisualTimelineEvent::Approach { type_: ApproachType::Second, time, distance });
    }
}

fn generate_encounters(view: &View, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    for encounter in view.model.future_encounters(entity) {
        let time = encounter.time();
        match encounter.encounter_type() {
            EncounterType::Entrance => events.push(VisualTimelineEvent::Encounter { type_: EncounterType::Exit, time, other_entity: encounter.to() }),
            EncounterType::Exit => events.push(VisualTimelineEvent::Encounter { type_: EncounterType::Entrance, time, other_entity: encounter.from() }),
        }
    }
}

pub fn update(view: &View, ui: &mut Ui, entity: Entity) {
    let mut events = vec![];

    generate_timeline_events(view, entity, &mut events);
    generate_apoapsis_periapsis(view, entity, &mut events);
    generate_closest_approaches(view, entity, &mut events);
    generate_encounters(view, entity, &mut events);

    events.sort_by(|a, b| a.time().total_cmp(&b.time()));

    if let Some(last_event) = view.model.vessel_component(entity).timeline().last_event() {
        if last_event.is_intercept() {
            events.retain(|event| event.time() <= last_event.time());
        }
    }

    for event in events {
        ui.horizontal(|ui| {
            let image = CustomImage::new(view, event.icon(), 20.0)
                .with_padding(event.padding());
            ui.add(image);
            ui.label(RichText::new(format!("T- {}", format_time((event.time().ceil() - view.model.time()).floor()))).weak().size(12.0));

            let width = 150.0 - ui.cursor().left();
            let mut rect = ui.cursor();
            rect.set_width(width);
            rect.set_height(0.0);
            ui.advance_cursor_after_rect(rect);

            ui.label(RichText::new(event.name(view)));
            ui.end_row();
        });
    }
}