use eframe::egui::PointerState;
use log::trace;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::{path_component::orbit::Orbit, vessel_component::Faction, ComponentType}, storage::entity_allocator::Entity};

use crate::game::{events::ViewEvent, selected::Selected, util::{should_render_at_time, ApsisType}, View};

use super::Icon;

fn compute_time_of_next_periapsis(orbit: &Orbit) -> Option<f64> {
    let periapsis_time = if orbit.is_ellipse() {
        let mut periapsis_time = orbit.first_periapsis_time();
        while periapsis_time <= orbit.current_point().time() {
            periapsis_time += orbit.period().unwrap();
        }
        periapsis_time
    } else {
        orbit.first_periapsis_time()
    };

    if periapsis_time > orbit.current_point().time() && periapsis_time < orbit.end_point().time() {
        Some(periapsis_time)
    } else {
        None
    }
}

fn compute_time_of_next_apoapsis(orbit: &Orbit) -> Option<f64> {
    if !orbit.is_ellipse() {
        return None;
    }

    let mut apoapsis_time = orbit.first_periapsis_time() - orbit.period().unwrap() / 2.0;
    while apoapsis_time <= orbit.current_point().time() {
        apoapsis_time += orbit.period().unwrap();
    }
    if apoapsis_time > orbit.current_point().time() && apoapsis_time < orbit.end_point().time() {
        Some(apoapsis_time)
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Apsis {
    type_: ApsisType,
    entity: Entity,
    time: f64,
    position: DVec2,
}

impl Apsis {
    pub fn new(view: &View, type_: ApsisType, entity: Entity, orbit: &Orbit, time: f64) -> Self {
        let position = view.model.absolute_position(orbit.parent()) + orbit.point_at_time(time).position();
        Self { type_, entity, time, position }
    }
    
    pub fn generate_for_orbit(view: &View, entity: Entity, orbit: &Orbit, icons: &mut Vec<Box<dyn Icon>>) {
        if let Some(time) = compute_time_of_next_periapsis(orbit) {
            if should_render_at_time(view, entity, time) {
                let icon = Apsis::new(view, ApsisType::Periapsis, entity, orbit, time);
                icons.push(Box::new(icon) as Box<dyn Icon>);
            }
        }

        if let Some(time) = compute_time_of_next_apoapsis(orbit) {
            if should_render_at_time(view, entity, time) {
                let icon = Apsis::new(view, ApsisType::Apoapsis, entity, orbit, time);
                icons.push(Box::new(icon) as Box<dyn Icon>);
            }
        }
    }

    pub fn generate(view: &View) -> Vec<Box<dyn Icon>> {
        let mut icons = vec![];
        for entity in view.model.entities(vec![ComponentType::VesselComponent]) {
            for orbit in &view.model.future_orbits(entity, Some(Faction::Player)) {
                Self::generate_for_orbit(view, entity, orbit, &mut icons);
            }
        }

        for entity in view.model.entities(vec![ComponentType::OrbitableComponent]) {
            if let Some(orbit) = view.model.orbitable_component(entity).orbit() {
                Self::generate_for_orbit(view, entity, orbit, &mut icons);
            }
        }

        icons
    }
}

impl Icon for Apsis {
    fn texture(&self, _view: &View) -> String {
        match self.type_ {
            ApsisType::Periapsis => "periapsis",
            ApsisType::Apoapsis => "apoapsis",
        }.to_string()
    }

    fn alpha(&self, _view: &View, is_selected: bool, is_hovered: bool, is_overlapped: bool) -> f32 {
        if is_overlapped {
            return 0.4;
        }
        if is_selected {
            return 1.0;
        }
        if is_hovered {
            return 0.8
        }
        0.6
    }

    fn radius(&self, _view: &View) -> f64 {
        8.0
    }

    fn priorities(&self, view: &View) -> [u64; 4] {
        [
            u64::from(self.is_selected(view)),
            0,
            2,
            0,
        ]
    }

    fn position(&self, _view: &View) -> DVec2 {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Apoapsis position");
        self.position
    }

    fn facing(&self, _view: &View) -> Option<DVec2> {
        None
    }

    fn is_selected(&self, view: &View) -> bool {
        if let Selected::Apsis { type_, entity, time } = &view.selected {
            // calculations can produce slightly different times when an apsis is calculated
            *type_ == self.type_ && *entity == self.entity && (*time - self.time).abs() < 1.0
        } else {
            false
        }
    }

    fn on_mouse_over(&self, view: &View, pointer: &PointerState) {
        if pointer.primary_clicked() {
            trace!("Apsis icon clicked; switching to Selected");
            let selected = Selected::Apsis { type_: self.type_, entity: self.entity, time: self.time };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}