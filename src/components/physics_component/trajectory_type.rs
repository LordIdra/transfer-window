use nalgebra_glm::DVec2;

use super::trajectory::Trajectory;

pub enum PhysicsType {
    Stationary(DVec2),
    Trajectory(Trajectory),
}