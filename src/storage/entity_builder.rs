use crate::components::{name_component::NameComponent, mass_component::MassComponent, orbitable_component::OrbitableComponent, trajectory_component::TrajectoryComponent, stationary_component::StationaryComponent};

pub struct EntityBuilder {
    pub mass_component: Option<MassComponent>,
    pub name_component: Option<NameComponent>,
    pub orbitable_component: Option<OrbitableComponent>,
    pub stationary_component: Option<StationaryComponent>,
    pub trajectory_component: Option<TrajectoryComponent>,
}

/// Specifies how an entity should be built
/// To build, pass into the state's allocate function
impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            mass_component: None,
            name_component: None,
            orbitable_component: None,
            trajectory_component: None,
            stationary_component: None,
        }
    }

    pub fn with_mass_component(mut self, component: MassComponent) -> Self {
        self.mass_component = Some(component);
        self
    }

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
}