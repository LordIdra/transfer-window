pub mod mass_component;
pub mod name_component;
pub mod orbitable_component;
pub mod stationary_component;
pub mod trajectory_component;
pub mod vessel_component;

pub enum ComponentType {
    MassComponent,
    NameComponent,
    OrbitableComponent,
    TrajectoryComponent,
    StationaryComponent,
}