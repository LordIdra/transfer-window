pub mod name_component;
pub mod orbitable_component;
pub mod path_component;
pub mod vessel_component;

pub enum ComponentType {
    NameComponent,
    OrbitableComponent,
    PathComponent,
    VesselComponent,
}