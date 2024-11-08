#![allow(clippy::match_same_arms)]

use eframe::egui::{Color32, Frame, RichText, Ui};
use transfer_window_model::{components::{path_component::segment::Segment, vessel_component::{faction::Faction, timeline::TimelineEvent}}, model::{encounters::EncounterType, state_query::StateQuery}, storage::entity_allocator::Entity};

use crate::game::{events::ViewEvent, overlay::widgets::{custom_image::CustomImage, labels::{draw_subtitle, draw_value}, util::advance_cursor_to}, selected::{util::BurnState, Selected}, util::{format_distance, format_time, ApproachType, ApsisType}, View};

enum VisualTimelineEvent {
    TimelineEvent(TimelineEvent),
    Apsis { type_: ApsisType, time: f64, altitude: f64 },
    Approach { type_: ApproachType, target: Entity, time: f64, distance: f64 },
    Encounter { type_: EncounterType, time: f64, from: Entity, to: Entity },
    Point { time: f64 },
    BurnEnd { time: f64 },
    TurnEnd { time: f64 },
    GuidanceEnd { time: f64 },
}

impl VisualTimelineEvent {
    pub fn time(&self) -> f64 {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => event.time(),
            VisualTimelineEvent::Apsis { time, .. } 
                | VisualTimelineEvent::Approach { time, .. } 
                | VisualTimelineEvent::Point { time }
                | VisualTimelineEvent::Encounter { time, .. } 
                | VisualTimelineEvent::BurnEnd { time } 
                | VisualTimelineEvent::TurnEnd { time } 
                | VisualTimelineEvent::GuidanceEnd { time } => *time,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => match event {
                TimelineEvent::Intercept(_) => "intercept",
                TimelineEvent::FireTorpedo(_) => "torpedo",
                TimelineEvent::StartBurn(_) => "burn",
                TimelineEvent::StartTurn(_) => "turn",
                TimelineEvent::StartGuidance(_) => "enable-guidance",
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
            VisualTimelineEvent::BurnEnd { .. } => "timeline-burn-end",
            VisualTimelineEvent::TurnEnd { .. } => "timeline-turn-end",
            VisualTimelineEvent::GuidanceEnd { .. } => "timeline-guidance-end",
        }
    }

    pub fn name(&self, view: &View) -> String {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => match event {
                TimelineEvent::Intercept(_) => "Intercept".to_string(),
                TimelineEvent::FireTorpedo(_) => "Torpedo Launch".to_string(),
                TimelineEvent::StartBurn(_) => "Burn Start".to_string(),
                TimelineEvent::StartTurn(_) => "Turn Start".to_string(),
                TimelineEvent::StartGuidance(_) => "Guidance Start".to_string(),
            }
            VisualTimelineEvent::Apsis { type_, altitude: distance, .. } => match type_ {
                ApsisType::Periapsis => format!("Periapsis - {}", format_distance(*distance)),
                ApsisType::Apoapsis => format!("Apoapsis - {}", format_distance(*distance)),
            }
            VisualTimelineEvent::Approach { distance, .. } => format!("Approach - {}", format_distance(*distance)),
            VisualTimelineEvent::Encounter { type_, from, to, .. } => match type_ {
                EncounterType::Entrance => format!("{} Entrance", view.model.name_component(*to).name()),
                EncounterType::Exit => format!("{} Exit", view.model.name_component(*from).name()),
            }
            VisualTimelineEvent::Point { .. } => "Selected point".to_string(),
            VisualTimelineEvent::BurnEnd { .. } => "Burn end".to_string(),
            VisualTimelineEvent::TurnEnd { .. } => "Turn end".to_string(),
            VisualTimelineEvent::GuidanceEnd { .. } => "Guidance end".to_string(),
        }
    }

    pub fn selected(&self, entity: Entity) -> Option<Selected> {
        match self {
            VisualTimelineEvent::TimelineEvent(event) => Some(match event {
                TimelineEvent::Intercept(intercept) => Selected::Intercept { entity, time: intercept.time() },
                TimelineEvent::FireTorpedo(fire_torpedo) => Selected::FireTorpedo { entity, time: fire_torpedo.time(), state: BurnState::Selected },
                TimelineEvent::StartBurn(burn) => Selected::Burn { entity, time: burn.time(), state: BurnState::Selected },
                TimelineEvent::StartTurn(turn) => Selected::Turn { entity, time: turn.time() },
                TimelineEvent::StartGuidance(enable_guidance) => Selected::EnableGuidance { entity, time: enable_guidance.time() },
            }),
            VisualTimelineEvent::Apsis { type_, time, altitude: _ } => Some(Selected::Apsis { type_: *type_, entity, time: *time }),
            VisualTimelineEvent::Approach { type_, target, time, distance: _ } => Some(Selected::Approach { type_: *type_, entity, target: *target, time: *time }),
            VisualTimelineEvent::Encounter { type_, time, from, to } => Some(Selected::Encounter { type_: *type_, entity, time: *time, from: *from, to: *to }),
            VisualTimelineEvent::Point { .. } 
                | VisualTimelineEvent::BurnEnd { .. } 
                | VisualTimelineEvent::TurnEnd { .. } 
                | VisualTimelineEvent::GuidanceEnd { .. } => None,
        }
    }

    pub fn is_selected(&self, view: &View, other_entity: Entity) -> bool {
        if view.selected.entity(&view.model).is_some_and(|entity| entity != other_entity) {
            return false;
        };

        match view.selected {
            Selected::None => false,
            Selected::Orbitable(_) => false,
            Selected::Vessel(_) => false,
            Selected::BurnPoint { .. } 
                | Selected::TurnPoint { .. } 
                | Selected::GuidancePoint { .. } 
                | Selected::OrbitPoint { .. } => matches!(self, VisualTimelineEvent::Point { .. }),
            Selected::Apsis { type_, time, .. } => {
                match self {
                    // calculations can produce slightly different times when an apsis is calculated
                    VisualTimelineEvent::Apsis { type_: other_type, time: other_time, .. } => type_ == *other_type && (time - *other_time).abs() < 1.0,
                    _ => false,
                }
            }
            Selected::Approach { type_, .. } => {
                match self {
                    VisualTimelineEvent::Approach { type_: other_type, .. } => type_ == *other_type,
                    _ => false,
                }
            }
            Selected::Encounter { type_, from, to, time, .. } => {
                match self {
                    // calculations can produce slightly different times when encounters calculated
                    VisualTimelineEvent::Encounter { type_: other_type, from: other_from, to: other_to, time: other_time, } => type_ == *other_type && from == *other_from && to == *other_to && (time - *other_time).abs() < 1.0,
                    _ => false,
                }
            }
            Selected::Intercept { .. } => match self {
                VisualTimelineEvent::TimelineEvent(event) => event.is_intercept(), // only 1 intercept at a time is possible (in theory)
                _ => false,
            }
            Selected::Burn { time, .. } => match self {
                VisualTimelineEvent::TimelineEvent(TimelineEvent::StartBurn(event)) => event.time() == time,
                _ => false,
            }
            Selected::Turn { time, .. } => match self {
                VisualTimelineEvent::TimelineEvent(TimelineEvent::StartTurn(event)) => event.time() == time,
                _ => false,
            }
            Selected::FireTorpedo { time, .. } => match self {
                VisualTimelineEvent::TimelineEvent(TimelineEvent::FireTorpedo(event)) => event.time() == time,
                _ => false,
            }
            Selected::EnableGuidance { time, .. } => match self {
                VisualTimelineEvent::TimelineEvent(TimelineEvent::StartGuidance(event)) => event.time() == time,
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
    for orbit in view.model.snapshot_now_observe(Faction::Player).future_orbits(entity) {
        if let Some(time) = orbit.next_periapsis_time() {
            let altitude = view.model.snapshot_at_observe(time, Faction::Player).surface_altitude(entity);
            events.push(VisualTimelineEvent::Apsis { type_: ApsisType::Periapsis, time, altitude });
        }

        if let Some(time) = orbit.next_apoapsis_time() {
            let altitude = view.model.snapshot_at_observe(time, Faction::Player).surface_altitude(entity);
            events.push(VisualTimelineEvent::Apsis { type_: ApsisType::Apoapsis, time, altitude });
        }
    }
}

fn generate_closest_approaches(view: &View, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    let Some(target) = view.model.vessel_component(entity).target() else { 
        return;
    };

    let (approach_1_time, approach_2_time) = view.model.snapshot_now_observe(Faction::Player).find_next_two_closest_approaches(entity, target);

    if let Some(time) = approach_1_time {
        let distance = view.model.snapshot_at_observe(time, Faction::Player).distance(entity, target);
        events.push(VisualTimelineEvent::Approach { type_: ApproachType::First, target, time, distance });
    }

    if let Some(time) = approach_2_time {
        let distance = view.model.snapshot_at_observe(time, Faction::Player).distance(entity, target);
        events.push(VisualTimelineEvent::Approach { type_: ApproachType::Second, target, time, distance });
    }
}

fn generate_encounters(view: &View, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    for encounter in view.model.snapshot_now_observe(Faction::Player).future_encounters(entity) {
        events.push(VisualTimelineEvent::Encounter { type_: encounter.encounter_type(), time: encounter.time(), from: encounter.from(), to: encounter.to() });
    }
}

fn generate_segment_end(view: &View, entity: Entity, events: &mut Vec<VisualTimelineEvent>) {
    if let Segment::Burn(burn) = view.model.path_component(entity).current_segment() {
        events.push(VisualTimelineEvent::BurnEnd { time: burn.end_point().time() });
    }

    if let Segment::Turn(turn) = view.model.path_component(entity).current_segment() {
        events.push(VisualTimelineEvent::TurnEnd { time: turn.end_point().time() });
    }

    if let Segment::Guidance(guidance) = view.model.path_component(entity).current_segment() {
        if !guidance.will_intercept() {
            events.push(VisualTimelineEvent::GuidanceEnd { time: guidance.end_point().time() });
        }
    }
}

pub fn draw_visual_timeline(view: &View, ui: &mut Ui, entity: Entity, center_time: f64, draw_center_time_point: bool) {
    if !view.config.draw_timeline {
        return;
    }
    
    let mut events = vec![];
    let faction = view.model.vessel_component(entity).faction();
    let has_intel = Faction::Player.has_intel_for(faction);

    if has_intel {
        generate_timeline_events(view, entity, &mut events);
    }
    generate_apoapsis_periapsis(view, entity, &mut events);
    if has_intel {
        generate_closest_approaches(view, entity, &mut events);
    }
    generate_encounters(view, entity, &mut events);
    if has_intel {
        generate_segment_end(view, entity, &mut events);
    }

    if draw_center_time_point {
        events.push(VisualTimelineEvent::Point { time: center_time });
    }

    events.sort_by(|a, b| a.time().total_cmp(&b.time()));

    if let Some(last_event) = view.model.vessel_component(entity).timeline().last_event() {
        if has_intel && last_event.is_intercept() {
            events.retain(|event| event.time() <= last_event.time());
        }
    }

    draw_subtitle(ui, "Timeline");
    for event in events {
        draw_event(view, ui, &event, entity, center_time);
    }
}

fn draw_event(view: &View, ui: &mut Ui, event: &VisualTimelineEvent, entity: Entity, center_time: f64) {
    let time_until = (event.time().ceil() - center_time).floor();

    let mut frame = Frame::default().begin(ui);

    frame.content_ui.horizontal(|ui| {
        ui.style_mut().visuals.panel_fill = Color32::RED;
        let image = CustomImage::new(view, event.icon(), 20);
        ui.add(image);

        let time_text = if event.is_selected(view, entity) {
            "now".to_string()
        } else if time_until.is_sign_positive() {
            format!("T-{}", format_time(time_until))
        } else {
            format!("T+{}", format_time(-time_until))
        };
        ui.label(RichText::new(time_text).weak().size(12.0));
        advance_cursor_to(ui, 150.0);

        draw_value(ui, &event.name(view));
        advance_cursor_to(ui, 320.0);
    });

    let response = frame.allocate_space(ui);
    let alpha = if response.hovered() { 255 } else { 180 };
    let fill = if event.is_selected(view, entity) {
        Color32::from_rgba_unmultiplied(35, 35, 60, alpha)
    } else if time_until.is_sign_positive() {
        Color32::from_rgba_unmultiplied(30, 30, 30, alpha)
    } else {
        Color32::from_rgba_unmultiplied(20, 20, 20, alpha)
    };

    // Annoying workaround because allocate_space only uses hover sense for some reason
    if response.contains_pointer() && view.context.input(|input| input.pointer.primary_clicked()) {
        if let Some(selected) = event.selected(entity) {
            view.add_view_event(ViewEvent::SetSelected(selected));
        }
    }
    
    frame.frame.fill = fill;

    frame.paint(ui);
}
