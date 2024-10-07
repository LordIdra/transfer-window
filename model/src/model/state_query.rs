use nalgebra_glm::DVec2;

use crate::{components::path_component::{burn::Burn, guidance::Guidance, orbit::Orbit, segment::Segment, turn::Turn}, storage::entity_allocator::Entity};

use super::encounters::Encounter;

pub trait StateQuery {
    fn future_segments(&self, entity: Entity) -> Vec<&Segment>;
    fn future_orbits(&self, entity: Entity) -> Vec<&Orbit>;
    fn future_burns(&self, entity: Entity) -> Vec<&Burn>;
    fn future_turns(&self, entity: Entity) -> Vec<&Turn>;
    fn future_guidances(&self, entity: Entity) -> Vec<&Guidance>;

    fn segment(&self, entity: Entity) -> &Segment;
    fn orbit(&self, entity: Entity) -> &Orbit;
    fn burn(&self, entity: Entity) -> &Burn;
    fn turn(&self, entity: Entity) -> &Turn;
    fn guidance(&self, entity: Entity) -> &Guidance;

    fn parent(&self, entity: Entity) -> Option<Entity>;
    fn target(&self, entity: Entity) -> Option<Entity>;
    fn rotation(&self, entity: Entity) -> f64;
    fn displacement(&self, entity: Entity, other_entity: Entity) -> DVec2;
    fn distance(&self, entity: Entity, other_entity: Entity) -> f64;
    fn surface_altitude(&self, entity: Entity) -> f64;
    fn position(&self, entity: Entity) -> DVec2;
    fn absolute_position(&self, entity: Entity) -> DVec2;
    fn velocity(&self, entity: Entity) -> DVec2;
    fn absolute_velocity(&self, entity: Entity) -> DVec2;
    fn relative_velocity(&self, entity: Entity, other_entity: Entity) -> DVec2;
    fn relative_speed(&self, entity: Entity, other_entity: Entity) -> f64;
    fn mass(&self, entity: Entity) -> f64;
    fn fuel_kg(&self, entity: Entity) -> f64;

    fn end_fuel(&self, entity: Entity) -> Option<f64>;
    fn end_dv(&self, entity: Entity) -> Option<f64>;

    fn find_next_closest_approach(&self, entity_a: Entity, entity_b: Entity) -> Option<f64>;
    fn find_next_two_closest_approaches(&self, entity_a: Entity, entity_b: Entity) -> (Option<f64>, Option<f64>);

    fn future_encounters(&self, entity: Entity) -> Vec<Encounter>;
}
