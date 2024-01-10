use self::trajectory_type::PhysicsType;

mod trajectory;
mod trajectory_type;

pub struct TrajectoryComponent {
    mass: f64,
    trajectory: PhysicsType,
}