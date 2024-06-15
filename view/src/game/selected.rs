use transfer_window_model::{api::encounters::EncounterType, storage::entity_allocator::Entity, Model};

use self::util::BurnState;

use super::util::{ApproachType, ApsisType};

pub mod burn;
pub mod fire_torpedo;
pub mod segment_point;
pub mod util;



#[derive(Debug, Clone)]
pub enum Selected {
    None,
    Orbitable(Entity),
    Vessel(Entity),
    BurnPoint { entity: Entity, time: f64 },
    GuidancePoint { entity: Entity, time: f64 },
    OrbitPoint { entity: Entity, time: f64 },
    Apsis { type_: ApsisType, entity: Entity, time: f64 },
    Approach { type_: ApproachType, entity: Entity, target: Entity, time: f64 },
    Encounter { type_: EncounterType, entity: Entity, time: f64, from: Entity, to: Entity },
    Intercept { entity: Entity, time: f64 },
    Burn { entity: Entity, time: f64, state: BurnState },
    FireTorpedo { entity: Entity, time: f64, state: BurnState },
    EnableGuidance { entity: Entity, time: f64 },
}

impl Selected {
    pub fn entity(&self, model: &Model) -> Option<Entity> {
        match self {
            Selected::None => None,
            Selected::FireTorpedo { entity, time, state: _ } => Some(model.fire_torpedo_event_at_time(*entity, *time).expect("No fire torpedo event at time").ghost()),
            Selected::Orbitable(entity) 
                | Selected::Vessel(entity) 
                | Selected::Burn { entity, .. }
                | Selected::BurnPoint { entity, .. } 
                | Selected::GuidancePoint { entity, .. } 
                | Selected::OrbitPoint { entity, .. } 
                | Selected::Apsis { entity, .. }
                | Selected::Approach {  entity, .. }
                | Selected::Encounter { entity, .. }
                | Selected::Intercept { entity, .. }
                | Selected::EnableGuidance { entity, .. } => Some(*entity),
        }
    }

    pub fn time(&self) -> Option<f64> {
        match self {
            Selected::None
                | Selected::Orbitable(_) 
                | Selected::Vessel(_) => None,
            Selected::FireTorpedo { time, .. }
                | Selected::Burn { time, .. }
                | Selected::BurnPoint { time, .. } 
                | Selected::GuidancePoint { time, .. } 
                | Selected::OrbitPoint { time, .. } 
                | Selected::Apsis { time, .. }
                | Selected::Approach { time, .. }
                | Selected::Encounter { time, .. }
                | Selected::Intercept { time, .. }
                | Selected::EnableGuidance { time, .. } => Some(*time),
        }
    }

    pub fn target(&self, model: &Model) -> Option<Entity> {
        if let Some(entity) = self.entity(model) {
            if let Some(vessel_component) = model.try_vessel_component(entity) {
                return vessel_component.target();
            }
        }
        None
    }
}