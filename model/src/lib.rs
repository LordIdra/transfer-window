use std::collections::HashSet;

use components::{trajectory_component::segment::Segment, vessel_component::VesselComponent};
use log::error;
use modules::trajectory_prediction;
use nalgebra_glm::{vec2, DVec2};
use serde::{Deserialize, Serialize};
use systems::{time::{self, TimeStep}, trajectory_update, warp_update_system::{self, TimeWarp}};
use util::find_closest_point_on_orbit;

use self::{components::{mass_component::MassComponent, name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::TrajectoryComponent, ComponentType}, storage::{component_storage::ComponentStorage, entity_allocator::{Entity, EntityAllocator}, entity_builder::EntityBuilder}};

pub const SEGMENTS_TO_PREDICT: usize = 3;

pub mod components;
mod debug;
mod modules;
pub mod storage;
mod systems;
mod util;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    entity_allocator: EntityAllocator,
    mass_components: ComponentStorage<MassComponent>,
    name_components: ComponentStorage<NameComponent>,
    orbitable_components: ComponentStorage<OrbitableComponent>,
    stationary_components: ComponentStorage<StationaryComponent>,
    trajectory_components: ComponentStorage<TrajectoryComponent>,
    vessel_components: ComponentStorage<VesselComponent>,
    time: f64,
    time_step: TimeStep,
    warp: Option<TimeWarp>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            entity_allocator: EntityAllocator::default(),
            mass_components: ComponentStorage::default(),
            name_components: ComponentStorage::default(),
            orbitable_components: ComponentStorage::default(),
            trajectory_components: ComponentStorage::default(),
            stationary_components: ComponentStorage::default(),
            vessel_components: ComponentStorage::default(),
            time: 0.0,
            time_step: TimeStep::Level{ level: 1, paused: false },
            warp: None,
        }
    }
}

impl Model {

    /// # Errors
    /// Forwards serde deserialization error if deserialization fails
    pub fn deserialize(serialized: &str) -> Result<Self, String> {
        match serde_json::from_str(serialized) {
            Ok(model) => Ok(model),
            Err(error) => Err(error.to_string()),
        }
    }

    /// # Errors
    /// Forwards serde serialization error if serialization fails
    pub fn serialize(&self) -> Result<String, String> {
        match serde_json::to_string(self) {
            Ok(serialized) => Ok(serialized),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn update(&mut self, dt: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Model update");
        warp_update_system::update(self, dt);
        time::update(self, dt);
        trajectory_update::update(self, dt);
    }

    pub fn toggle_paused(&mut self) {
        self.time_step.toggle_paused();
    }

    pub fn increase_time_step_level(&mut self) {
        self.time_step.increase_level();
    }

    pub fn decrease_time_step_level(&mut self) {
        self.time_step.decrease_level();
    }

    pub fn start_warp(&mut self, end_time: f64) {
        self.warp = Some(TimeWarp::new(self.time, end_time));
    }

    pub fn get_time_step(&self) -> &TimeStep {
        &self.time_step
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_position(&self, entity: Entity) -> Option<DVec2> {
        if let Some(stationary_component) = self.try_get_stationary_component(entity) {
            return Some(stationary_component.get_position())
        }

        if let Some(trajectory_component) = self.try_get_trajectory_component(entity) {
            return Some(trajectory_component.get_current_segment().get_current_position())
        }

        None
    }

    /// Returns the entity and time of the closest point on ANY segment anywhere provided the closest
    /// distance from the point to a segment is less than `max_distance`
    /// Short circuits; if there are multiple points, the first one found is returned
    pub fn get_closest_point_on_trajectory(&self, point: DVec2, max_distance: f64) -> Option<(Entity, f64)> {
        let mut closest_point = None;
        let mut closest_distance = f64::MAX;
        for entity in self.get_entities(vec![ComponentType::TrajectoryComponent]) {
            for segment in self.get_trajectory_component(entity).get_segments().iter().flatten() {
                let Segment::Orbit(orbit) = segment else { 
                    continue 
                };
                
                let parent_position = self.get_absolute_position(orbit.get_parent());
                let point = point - parent_position;
                let Some(closest_position) = find_closest_point_on_orbit(orbit, point, max_distance) else {
                    continue;
                };

                let distance = (closest_position - point).magnitude();
                let theta = f64::atan2(closest_position.y, closest_position.x);
                let time = orbit.get_first_periapsis_time() + orbit.get_time_since_first_periapsis(theta);
                if distance > closest_distance {
                    continue
                }

                closest_distance = distance;
                if time > orbit.get_current_point().get_time() && time < orbit.get_end_point().get_time() {
                    closest_point = Some((entity, time));
                    closest_distance = distance;
                    continue;
                }
                
                if let Some(period) = orbit.get_period() {
                    // If the orbit has a period, we might have calculated an invalid time that's one period behind a valid time
                    let time = time + period;
                    if time > orbit.get_current_point().get_time() && time < orbit.get_end_point().get_time() {
                        closest_point = Some((entity, time));
                        closest_distance = distance;
                    }
                }
            }
        }

        closest_point
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn get_absolute_position(&self, entity: Entity) -> DVec2 {
        if let Some(trajectory_component) = self.try_get_trajectory_component(entity) {
            let current_segment = trajectory_component.get_current_segment();
            return self.get_absolute_position(current_segment.get_parent()) + current_segment.get_current_position();
        }

        if let Some(stationary_component) = self.try_get_stationary_component(entity) {
            return stationary_component.get_position();
        }

        error!("Request to get absolute position of entity without trajectory or stationary components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn get_absolute_velocity(&self, entity: Entity) -> DVec2 {
        if let Some(trajectory_component) = self.try_get_trajectory_component(entity) {
            let current_segment = trajectory_component.get_current_segment();
            return self.get_absolute_velocity(current_segment.get_parent()) + current_segment.get_current_velocity();
        }

        if self.try_get_stationary_component(entity).is_some() {
            return vec2(0.0, 0.0);
        }

        error!("Request to get absolute position of entity without trajectory or stationary components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    pub fn get_entities(&self, mut with_component_types: Vec<ComponentType>) -> HashSet<Entity> {
        let mut entities = self.entity_allocator.get_entities().clone();
        while let Some(component_type) = with_component_types.pop() {
            let other_entities = match component_type {
                ComponentType::MassComponent => self.mass_components.get_entities(),
                ComponentType::NameComponent => self.name_components.get_entities(),
                ComponentType::OrbitableComponent => self.orbitable_components.get_entities(),
                ComponentType::StationaryComponent => self.stationary_components.get_entities(),
                ComponentType::TrajectoryComponent => self.trajectory_components.get_entities(),
                ComponentType::VesselComponent => self.vessel_components.get_entities(),
            };
            entities.retain(|entity| other_entities.contains(entity));
        }
        entities
    }

    pub fn predict(&mut self, entity: Entity, end_time: f64, segment_count: usize) {
        trajectory_prediction::predict(self, entity, end_time, segment_count);
    }

    pub fn allocate(&mut self, entity_builder: EntityBuilder) -> Entity {
        let EntityBuilder {
            mass_component,
            name_component,
            orbitable_component,
            stationary_component,
            trajectory_component,
            vessel_component,
        } = entity_builder;
        let entity = self.entity_allocator.allocate();
        self.mass_components.set(entity, mass_component);
        self.name_components.set(entity, name_component);
        self.orbitable_components.set(entity, orbitable_component);
        self.stationary_components.set(entity, stationary_component);
        self.trajectory_components.set(entity, trajectory_component);
        self.vessel_components.set(entity, vessel_component);
        entity
    }

    pub fn deallocate(&mut self, entity: Entity) {
        self.entity_allocator.deallocate(entity);
        self.name_components.remove_if_exists(entity);
    }

    pub fn get_mass_component_mut(&mut self, entity: Entity) -> &mut MassComponent {
        self.mass_components.get_mut(entity)
    }

    pub fn get_mass_component(&self, entity: Entity) -> &MassComponent {
        self.mass_components.get(entity)
    }

    pub fn try_get_mass_component_mut(&mut self, entity: Entity) -> Option<&mut MassComponent> {
        self.mass_components.try_get_mut(entity)
    }

    pub fn try_get_mass_component(&self, entity: Entity) -> Option<&MassComponent> {
        self.mass_components.try_get(entity)
    }

    pub fn get_name_component_mut(&mut self, entity: Entity) -> &mut NameComponent {
        self.name_components.get_mut(entity)
    }

    pub fn get_name_component(&self, entity: Entity) -> &NameComponent {
        self.name_components.get(entity)
    }

    pub fn try_get_name_component_mut(&mut self, entity: Entity) -> Option<&mut NameComponent> {
        self.name_components.try_get_mut(entity)
    }

    pub fn try_get_name_component(&self, entity: Entity) -> Option<&NameComponent> {
        self.name_components.try_get(entity)
    }

    pub fn get_stationary_component_mut(&mut self, entity: Entity) -> &mut StationaryComponent {
        self.stationary_components.get_mut(entity)
    }

    pub fn get_stationary_component(&self, entity: Entity) -> &StationaryComponent {
        self.stationary_components.get(entity)
    }

    pub fn try_get_stationary_component_mut(&mut self, entity: Entity) -> Option<&mut StationaryComponent> {
        self.stationary_components.try_get_mut(entity)
    }

    pub fn try_get_stationary_component(&self, entity: Entity) -> Option<&StationaryComponent> {
        self.stationary_components.try_get(entity)
    }

    pub fn get_orbitable_component_mut(&mut self, entity: Entity) -> &mut OrbitableComponent {
        self.orbitable_components.get_mut(entity)
    }

    pub fn get_orbitable_component(&self, entity: Entity) -> &OrbitableComponent {
        self.orbitable_components.get(entity)
    }

    pub fn try_get_orbitable_component_mut(&mut self, entity: Entity) -> Option<&mut OrbitableComponent> {
        self.orbitable_components.try_get_mut(entity)
    }

    pub fn try_get_orbitable_component(&self, entity: Entity) -> Option<&OrbitableComponent> {
        self.orbitable_components.try_get(entity)
    }

    pub fn get_trajectory_component_mut(&mut self, entity: Entity) -> &mut TrajectoryComponent {
        self.trajectory_components.get_mut(entity)
    }

    pub fn get_trajectory_component(&self, entity: Entity) -> &TrajectoryComponent {
        self.trajectory_components.get(entity)
    }

    pub fn try_get_trajectory_component_mut(&mut self, entity: Entity) -> Option<&mut TrajectoryComponent> {
        self.trajectory_components.try_get_mut(entity)
    }

    pub fn try_get_trajectory_component(&self, entity: Entity) -> Option<&TrajectoryComponent> {
        self.trajectory_components.try_get(entity)
    }

    pub fn get_vessel_component_mut(&mut self, entity: Entity) -> &mut VesselComponent {
        self.vessel_components.get_mut(entity)
    }

    pub fn get_vessel_component(&self, entity: Entity) -> &VesselComponent {
        self.vessel_components.get(entity)
    }

    pub fn try_get_vessel_component_mut(&mut self, entity: Entity) -> Option<&mut VesselComponent> {
        self.vessel_components.try_get_mut(entity)
    }

    pub fn try_get_vessel_component(&self, entity: Entity) -> Option<&VesselComponent> {
        self.vessel_components.try_get(entity)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{components::{name_component::NameComponent, ComponentType}, storage::entity_builder::EntityBuilder};

    use super::Model;

    #[test]
    fn test_components() {
        let mut model = Model::default();
        let builder1 = EntityBuilder::default().with_name_component(NameComponent::new("oh no".to_string()));
        let builder2 = EntityBuilder::default();
        let e1 = model.allocate(builder1);
        let e2 = model.allocate(builder2);

        let mut expected = HashSet::new();
        expected.insert(e1);
        expected.insert(e2);
        assert!(model.get_entities(vec![]) == expected);

        let mut expected = HashSet::new();
        expected.insert(e1);
        assert!(model.get_entities(vec![ComponentType::NameComponent]) == expected);
    }
}