use eframe::egui::{Context, RichText, Ui};
use thousands::Separable;
use transfer_window_model::{components::vessel_component::timeline::TimelineEvent, storage::entity_allocator::Entity, Model};

use crate::game::{overlay::widgets::custom_image::CustomImage, util::{format_time, EncounterType}, Scene};

fn format_distance(distance: f64) -> String {
    if distance < 1000.0 {
        format!("{} m", distance.round())
    } else if distance < 10000.0 {
        format!("{:.2} km", (distance / 1000.0))
    } else if distance < 100000.0 {
        format!("{:.1} km", (distance / 1000.0))
    } else {
        format!("{} km", (distance / 1000.0).round().separate_with_commas())
    }
}

enum VisualTimelineEvent {
    TimelineEvent(TimelineEvent),
    Periapsis { time: f64, distance: f64 },
    Apoapsis { time: f64, distance: f64 },
    FirstApproach { time: f64, distance: f64 },
    SecondApproach { time: f64, distance: f64 },
    EntranceEncounter { time: f64, type_: EncounterType, other_entity: Entity },
    ExitEncounter { time: f64, type_: EncounterType, other_entity: Entity },
}

impl VisualTimelineEvent {
    pub fn time(&self) -> f64 {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => event.time(),
            VisualTimelineEvent::Periapsis { time, .. } 
                | VisualTimelineEvent::Apoapsis { time, .. } 
                | VisualTimelineEvent::FirstApproach { time, .. } 
                | VisualTimelineEvent::SecondApproach { time, .. } 
                | VisualTimelineEvent::EntranceEncounter { time, .. }
                | VisualTimelineEvent::ExitEncounter { time, .. } => *time,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => match event {
                TimelineEvent::Intercept(_) => "intercept",
                TimelineEvent::FireTorpedo(_) => "torpedo",
                TimelineEvent::Burn(_) => "burn",
                TimelineEvent::EnableGuidance(_) => "enable-guidance",
            },
            VisualTimelineEvent::Periapsis { .. } => "periapsis",
            VisualTimelineEvent::Apoapsis { .. } => "apoapsis",
            VisualTimelineEvent::FirstApproach { .. } => "closest-approach-1",
            VisualTimelineEvent::SecondApproach { .. } => "closest-approach-2",
            VisualTimelineEvent::EntranceEncounter { .. } => "encounter-entrance",
            VisualTimelineEvent::ExitEncounter { .. } => "encounter-exit",
        }
    }

    pub fn padding(&self) -> f32 {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => match event {
                TimelineEvent::Intercept(_) => 3.0,
                TimelineEvent::FireTorpedo(_) => 0.0,
                TimelineEvent::Burn(_) => 0.0,
                TimelineEvent::EnableGuidance(_) => 0.0,
            },
            VisualTimelineEvent::Periapsis { .. } => 3.0,
            VisualTimelineEvent::Apoapsis { .. } => 3.0,
            VisualTimelineEvent::FirstApproach { .. } => 3.0,
            VisualTimelineEvent::SecondApproach { .. } => 3.0,
            VisualTimelineEvent::EntranceEncounter { .. } => 3.0,
            VisualTimelineEvent::ExitEncounter { .. } => 3.0,
        }
    }

    pub fn name(&self, model: &Model) -> String {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => match event {
                TimelineEvent::Intercept(_) => "Intercept".to_string(),
                TimelineEvent::FireTorpedo(_) => "Torpedo Launch".to_string(),
                TimelineEvent::Burn(_) => "Burn".to_string(),
                TimelineEvent::EnableGuidance(_) => "Guidance Enabled".to_string(),
            },
            VisualTimelineEvent::Periapsis { distance, .. } => format!("Periapsis - {}", format_distance(*distance)),
            VisualTimelineEvent::Apoapsis { distance, .. } => format!("Apoapsis - {}", format_distance(*distance)),
            VisualTimelineEvent::FirstApproach { distance, .. } => format!("Closest Approach - {}", format_distance(*distance)),
            VisualTimelineEvent::SecondApproach { distance, .. } => format!("Closest Approach - {}", format_distance(*distance)),
            VisualTimelineEvent::EntranceEncounter { other_entity, .. } => format!("{} Entrance", model.name_component(*other_entity).name()),
            VisualTimelineEvent::ExitEncounter { other_entity, .. } => format!("{} Exit", model.name_component(*other_entity).name()),
        }
    }
}

fn generate_timeline_events(model: &Model, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    for event in model.vessel_component(entity).timeline().events() {
        events.push(VisualTimelineEvent::TimelineEvent(event.clone()));
    }
}

fn generate_apoapsis_periapsis(model: &Model, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    for orbit in model.path_component(entity).future_orbits() {
        if let Some(time) = orbit.next_periapsis_time() {
            let distance = model.position_at_time(entity, time).magnitude();
            events.push(VisualTimelineEvent::Periapsis { time, distance });
        }

        if let Some(time) = orbit.next_apoapsis_time() {
            let distance = model.position_at_time(entity, time).magnitude();
            events.push(VisualTimelineEvent::Apoapsis { time: time, distance });
        }
    }
}

fn generate_closest_approaches(model: &Model, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    let Some(target) = model.vessel_component(entity).target() else { 
        return;
    };

    let (approach_1_time, approach_2_time) = model.find_next_two_closest_approaches(entity, target);

    if let Some(time) = approach_1_time {
        let distance = model.distance_at_time(entity, target, time);
        events.push(VisualTimelineEvent::FirstApproach { time, distance });
    }

    if let Some(time) = approach_2_time {
        let distance = model.distance_at_time(entity, target, time);
        events.push(VisualTimelineEvent::SecondApproach { time, distance });
    }
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, ui: &mut Ui, entity: Entity) {
    let mut events = vec![];

    generate_timeline_events(model, entity, &mut events);
    generate_apoapsis_periapsis(model, entity, &mut events);
    generate_closest_approaches(model, entity, &mut events);

    events.sort_by(|a, b| a.time().total_cmp(&b.time()));

    if let Some(last_event) = model.vessel_component(entity).timeline().last_event() {
        if last_event.is_intercept() {
            events.retain(|event| event.time() <= last_event.time());
        }
    }

    for event in events {
        ui.horizontal(|ui| {
            let image = CustomImage::new(view.renderers.get_screen_texture_renderer(event.icon()), context.screen_rect(), 24.0)
                .with_padding(event.padding());
            ui.add(image);
            ui.label(RichText::new(format!("T- {}", format_time((event.time().floor() - model.time()).floor()))).weak().size(12.0));

            let width = 150.0 - ui.cursor().left();
            let mut rect = ui.cursor();
            rect.set_width(width);
            rect.set_height(0.0);
            ui.advance_cursor_after_rect(rect);

            ui.label(RichText::new(event.name(model)));
            ui.end_row();
        });
    }
}