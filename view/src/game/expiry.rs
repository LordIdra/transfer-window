use log::trace;
use transfer_window_model::components::{path_component::segment::Segment, vessel_component::Faction};

use super::{selected::Selected, util::{should_render, should_render_at_time, ApsisType}, View};


pub fn update(view: &mut View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update expiry");

    // Unfocus camera if focus no longer exists
    if let Some(entity) = view.camera.focus() {
        if !view.model.entity_exists(entity) {
            view.camera.unset_focus();
        }
    }

    // Remove selected if expired
    if let Some(time) = view.selected.time() {
        if time < view.model.time() {
            trace!("Selected expired at time={time}");
            view.selected = Selected::None;
        }
    }

    // Remove selected approach if target is no longer targeted
    if let Selected::Approach { type_: _, entity, target, time: _ } = view.selected.clone() {
        if !view.model.vessel_component(entity).has_target() || view.model.vessel_component(entity).target().unwrap() != target {
            trace!("Selected approach no longer has target");
            view.selected = Selected::None;
        }
    }

    // Remove or update selected apsis if apsis no longer exists or is in a different place
    if let Selected::Apsis { type_, entity, time } = view.selected.clone() {
        if let Segment::Orbit(orbit) = view.model.segment_at_time(entity, time, Some(Faction::Player)) {
            let expected_time = match type_ {
                ApsisType::Periapsis => orbit.next_periapsis_time(),
                ApsisType::Apoapsis => orbit.next_apoapsis_time(),
            };
            if let Some(expected_time) = expected_time {
                view.selected = Selected::Apsis { type_, entity, time: expected_time }
            } else {
                trace!("Selected apsis orbit no longer has an orbit");
                view.selected = Selected::None;
            }
        } else {
            trace!("Selected apsis is no longer on an orbit");
            view.selected = Selected::None;
        }
    }

    // Remove or update selected encounter if encounter no longer exists or is in a different place
    if let Selected::Encounter { type_, entity, time, from, to } = view.selected {
        let mut any_encounter_matches = false;
        for encounter in view.model.future_encounters(entity, Some(Faction::Player)) {
            if encounter.encounter_type() == type_ && encounter.from() == from && encounter.to() == to && (time - encounter.time()).abs() < 10.0 {
                any_encounter_matches = true;
            }
        }
        if !any_encounter_matches {
            view.selected = Selected::None;
        }
    }

    // Remove selected fire torpedo event if no longer exists
    if let Selected::FireTorpedo { entity, time, state: _ } = view.selected.clone() {
        if view.model.fire_torpedo_event_at_time(entity, time).is_none() {
            trace!("Selected fire torpedo event expired at time={time}");
            view.selected = Selected::None;
        }
    }

    // Delete selected if its entity no longer exists or should not be rendered
    if let Some(entity) = view.selected.entity(&view.model) {
        if !view.model.entity_exists(entity) {
            view.selected = Selected::None;
        }
    }

    // Deleted selected if its entity should not be rendered
    if let Some(entity) = view.selected.entity(&view.model) {
        let should_render = match view.selected.time() {
            Some(time) => should_render_at_time(view, entity, time),
            None => should_render(view, entity),
        };
        if !should_render {
            view.selected = Selected::None;
        }
    }
}