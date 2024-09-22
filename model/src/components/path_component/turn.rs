use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};
use transfer_window_common::normalize_angle;
use turn_plan::TurnPlan;
use turn_point::TurnPoint;

use crate::{components::vessel_component::rcs::Rcs, storage::entity_allocator::Entity};

use super::orbit::Orbit;

pub mod turn_plan;
pub mod turn_point;
pub mod builder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Turn {
    orbit: Orbit,
    turn_plan: TurnPlan,
    start_point: TurnPoint,
    current_point: TurnPoint,
    end_point: TurnPoint,
}

impl Turn {
    #[allow(clippy::too_many_arguments)]
    pub fn new(parent: Entity, parent_mass: f64, dry_mass: f64, fuel_kg: f64, time: f64, position: DVec2, velocity: DVec2, rotation: f64, target_rotation: f64, rcs: &Rcs) -> Self {
        let mass = dry_mass + fuel_kg;
        let fuel_kg_per_second = rcs.turn_fuel_kg_per_second();
        let angular_acceleration = rcs.turn_force() / mass;
        let turn_plan = TurnPlan::new(angular_acceleration, fuel_kg_per_second, dry_mass, fuel_kg, time, rotation, target_rotation);
        let mut orbit = Orbit::new(parent, mass, parent_mass, rotation, position, velocity, time);
        orbit.end_at(turn_plan.end_time());
        let start_point = TurnPoint::new(time, position, velocity, rotation, mass);
        let current_point = start_point.clone();
        let end_point = TurnPoint::new(turn_plan.end_time(), orbit.end_point().position(), orbit.end_point().velocity(), target_rotation, turn_plan.end_mass());
        Self { orbit, turn_plan, start_point, current_point, end_point }
    }

    pub fn start_point(&self) -> &TurnPoint {
        &self.start_point
    }

    pub fn current_point(&self) -> &TurnPoint {
        &self.current_point
    }

    pub fn end_point(&self) -> &TurnPoint {
        &self.end_point
    }

    pub fn point_at_time(&self, time: f64) -> TurnPoint {
        let orbit_point = &self.orbit.point_at_time(time);
        TurnPoint::new(time, orbit_point.position(), orbit_point.velocity(), self.turn_plan.rotation_at_time(time), self.turn_plan.mass_at_time(time))
    }

    pub fn parent(&self) -> Entity {
        self.orbit.parent()
    }

    pub fn duration(&self) -> f64 {
        self.turn_plan.duration()
    }

    pub fn remaining_time(&self) -> f64 {
        self.end_point.time() - self.current_point.time()
    }

    pub fn angle(&self) -> f64 {
        normalize_angle(self.end_point.rotation() - self.start_point.rotation())
    }

    pub fn fuel_burnt(&self) -> f64 {
        self.start_fuel_kg() - self.end_fuel_kg()
    }

    pub fn is_finished(&self) -> bool {
        self.current_point.time() >= self.end_point().time()
    }

    pub fn is_time_within_turn(&self, time: f64) -> bool {
        time > self.start_point().time() && time < self.end_point().time()
    }

    pub fn start_fuel_kg(&self) -> f64 {
        self.turn_plan.start_fuel_kg()
    }

    pub fn fuel_kg_at_time(&self, time: f64) -> f64 {
        self.turn_plan.fuel_kg_at_time(time)
    }

    pub fn end_fuel_kg(&self) -> f64 {
        self.turn_plan.end_fuel_kg()
    }

    pub fn adjust(&mut self, amount: f64) {
        let dry_mass = self.turn_plan.dry_mass();
        let start_fuel_kg = self.turn_plan.start_fuel_kg();
        let start_time = self.start_point.time();
        let start_rotation = self.start_point.rotation();
        let end_rotation = self.turn_plan.end_rotation() + amount;

        self.turn_plan = TurnPlan::new(
            self.turn_plan.angular_acceleration(), 
            self.turn_plan.fuel_kg_per_second(),
            dry_mass,
            start_fuel_kg,
            start_time,
            start_rotation,
            end_rotation);

        self.orbit.end_at(self.turn_plan.end_time());
        self.end_point = TurnPoint::new(self.turn_plan.end_time(), self.orbit.end_point().position(), self.orbit.end_point().velocity(), end_rotation, self.turn_plan.end_mass());
    }

    pub fn next(&mut self, time: f64) {
        self.current_point = self.point_at_time(time);
    }
}
