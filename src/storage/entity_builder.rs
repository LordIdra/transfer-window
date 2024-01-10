use crate::components::{name_component::NameComponent, physics_component::PhysicsComponent};

pub struct EntityBuilder {
    pub name_component: Option<NameComponent>,
    pub physics_component: Option<PhysicsComponent>
}

/// Specifies how an entity should be built
/// To build, pass into the state's allocate function
impl EntityBuilder {
    pub fn new() -> Self {
        Self { 
            name_component: None,
            physics_component: None,
        }
    }

    pub fn with_name_component(mut self, component: NameComponent) -> Self {
        self.name_component = Some(component);
        self
    }

    pub fn with_physics_component(mut self, component: PhysicsComponent) -> Self {
        self.physics_component = Some(component);
        self
    }
}