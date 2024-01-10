use crate::components::name_component::NameComponent;

pub struct EntityBuilder {
    pub name_component: Option<NameComponent>,
}

/// Specifies how an entity should be built
/// To build, pass into the state's allocate function
impl EntityBuilder {
    pub fn new() -> Self {
        Self { 
            name_component: None,
        }
    }

    pub fn with_name_component(mut self, component: NameComponent) -> Self {
        self.name_component = Some(component);
        self
    }
}