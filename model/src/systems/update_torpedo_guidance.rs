use log::warn;
use nalgebra_glm::vec2;

use crate::{components::{vessel_component::{timeline::{start_burn::BurnEvent, TimelineEvent, TimelineEventType}, VesselClass}, ComponentType}, storage::entity_allocator::Entity, Model};

const LINE_OF_SIGHT_RATE_DELTA: f64 = 0.1;
const PROPORTIONALITY_CONSTANT: f64 = 3.0;

// https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=5217080
// https://gamedev.stackexchange.com/questions/118162/how-to-calculate-the-closing-speed-of-two-objects 
// https://en.wikipedia.org/wiki/Proportional_navigation
fn update_guidance(model: &mut Model, entity: Entity) {
    let Some(target) = model.vessel_component(entity).target() else {
        warn!("Torpedo did not have target so unable to perform guidance");
        return;
    };

    if model.path_component(entity).current_segment().is_burn() {
        return;
    }

    let absolute_velocity = model.absolute_velocity(entity);
    let target_absolute_velocity = model.absolute_velocity(target);
    let absolute_position = model.absolute_position(entity);
    let target_absolute_position = model.absolute_position(target);

    // https://gamedev.stackexchange.com/questions/118162/how-to-calculate-the-closing-speed-of-two-objects
    let displacement = absolute_position - target_absolute_position;
    let closing_speed = -(absolute_velocity - target_absolute_velocity).dot(&displacement) / displacement.magnitude();
    
    let future_displacement = model.absolute_position_at_time(entity, model.time + LINE_OF_SIGHT_RATE_DELTA) - model.absolute_position_at_time(target, model.time + LINE_OF_SIGHT_RATE_DELTA);
    let line_of_sight_rate = (f64::atan2(displacement.y, displacement.x) - f64::atan2(future_displacement.y, future_displacement.x)) / LINE_OF_SIGHT_RATE_DELTA;

    let acceleration_unit = vec2(-displacement.y, displacement.x).normalize();
    let acceleration = acceleration_unit * PROPORTIONALITY_CONSTANT * closing_speed * line_of_sight_rate;

    // TODO bounds checking to make sure we can actually create the burn
    let burn_time = model.time;
    let event = TimelineEventType::Burn(BurnEvent::new(model, entity, burn_time));
    model.add_event(entity, TimelineEvent::new(burn_time, event));
    let acceleration = model.burn_starting_at_time(entity, burn_time)
        .rotation_matrix()
        .try_inverse()
        .unwrap() * acceleration;
    model.event_at_time(entity, burn_time)
        .type_()
        .as_burn()
        .unwrap()
        .adjust(model, acceleration);
}

impl Model {
    pub(crate) fn update_torpedo_guidance(&mut self) {
        for entity in self.entities(vec![ComponentType::VesselComponent]) {
            let vessel_component = &self.vessel_component(entity);
            if let VesselClass::Torpedo(torpedo) = vessel_component.class() {
                if torpedo.guidance_enabled() {
                    update_guidance(self, entity);
                }
            }
        }
    }
}