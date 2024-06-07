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
    Point { entity: Entity, time: f64 },
    Apsis { type_: ApsisType, entity: Entity, time: f64 },
    Approach { type_: ApproachType, entity: Entity, time: f64 },
    Encounter { type_: EncounterType, entity: Entity, time: f64 },
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
                | Selected::Burn { entity, time: _, state: _ }
                | Selected::Point { entity, time: _ } 
                | Selected::Apsis { type_: _, entity, time: _ }
                | Selected::Approach { type_: _, entity, time: _ }
                | Selected::Encounter { type_: _, entity, time: _ }
                | Selected::EnableGuidance { entity, time: _ } => Some(*entity),
        }
    }

    pub fn time(&self) -> Option<f64> {
        match self {
            Selected::None
                | Selected::Orbitable(_) 
                | Selected::Vessel(_) => None,
            Selected::FireTorpedo { entity: _, time, state: _ }
                | Selected::Burn { entity: _, time, state: _ }
                | Selected::Point { entity: _, time } 
                | Selected::Apsis { type_: _, entity: _, time }
                | Selected::Approach { type_: _, entity: _, time }
                | Selected::Encounter { type_: _, entity: _, time }
                | Selected::EnableGuidance { entity: _, time } => Some(*time),
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