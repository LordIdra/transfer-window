pub mod name_component;
pub mod orbitable_component;
pub mod path_component;
pub mod vessel_component;
pub mod atmosphere_component;

#[derive(Debug, Clone, Copy)]
pub enum ComponentType {
    NameComponent,
    OrbitableComponent,
    AtmosphereComponent,
    PathComponent,
    VesselComponent,
}