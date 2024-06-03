use transfer_window_model::{storage::entity_allocator::Entity, Model};

use self::util::BurnState;

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