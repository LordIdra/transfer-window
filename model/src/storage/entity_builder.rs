use crate::components::{name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::TrajectoryComponent, vessel_component::VesselComponent};

#[derive(Debug, Default)]
pub struct EntityBuilder {
    pub name_component: Option<NameComponent>,
    pub orbitable_component: Option<OrbitableComponent>,
    pub stationary_component: Option<StationaryComponent>,
    pub trajectory_component: Option<TrajectoryComponent>,
    pub vessel_component: Option<VesselComponent>,
}

/// Specifies how an entity should be built
/// To build, pass into the model's allocate function
impl EntityBuilder {
    pub fn with_name_component(mut self, component: NameComponent) -> Self {
        self.name_component = Some(component);
        self
    }

    pub fn with_orbitable_component(mut self, component: OrbitableComponent) -> Self {
        self.orbitable_component = Some(component);
        self
    }

    pub fn with_stationary_component(mut self, component: StationaryComponent) -> Self {
        self.stationary_component = Some(component);
        self
    }

    pub fn with_trajectory_component(mut self, component: TrajectoryComponent) -> Self {
        self.trajectory_component = Some(component);
        self
    }

    pub fn with_vessel_component(mut self, component: VesselComponent) -> Self {
        self.vessel_component = Some(component);
        self
    }
}