use crate::{components::{name_component::NameComponent, orbitable_component::OrbitableComponent, path_component::PathComponent, vessel_component::{faction::Faction, VesselComponent}}, storage::entity_allocator::Entity};

use super::Model;

pub struct Snapshot {
    model: *const Model,
    time: f64,
    observer: Option<Faction>,
}

impl Snapshot {
    pub fn new(model: &Model, time: f64) -> Self {
        let model = model as *const Model;
        let observer = None;
        Self { model, time, observer }
    }

    pub fn model(&self) -> &Model {
        unsafe {
            &*self.model
        }
    }

    pub fn exists(&self, entity: Entity) -> bool {
        self.entity_allocator.entities().contains(&entity)
    }

    pub fn name_component(&self, entity: Entity) -> &NameComponent {
        assert!(self.exists(entity));
        self.model().name_components.get(entity)
    }

    pub fn try_name_component(&self, entity: Entity) -> Option<&NameComponent> {
        assert!(self.exists(entity));
        self.model().name_components.try_get(entity)
    }

    pub fn has_name_component(&self, entity: Entity) -> bool {
        assert!(self.exists(entity));
        self.try_name_component(entity).is_some()
    }

    pub fn orbitable_component(&self, entity: Entity) -> &OrbitableComponent {
        assert!(self.exists(entity));
        self.model().orbitable_components.get(entity)
    }

    pub fn try_orbitable_component(&self, entity: Entity) -> Option<&OrbitableComponent> {
        assert!(self.exists(entity));
        self.model().orbitable_components.try_get(entity)
    }

    pub fn has_orbitable_component(&self, entity: Entity) -> bool {
        assert!(self.exists(entity));
        self.try_orbitable_component(entity).is_some()
    }

    pub fn path_component(&self, entity: Entity) -> &PathComponent {
        assert!(self.exists(entity));
        self.model().path_components.get(entity)
    }

    pub fn try_path_component(&self, entity: Entity) -> Option<&PathComponent> {
        assert!(self.exists(entity));
        self.model().path_components.try_get(entity)
    }

    pub fn has_path_component(&self, entity: Entity) -> bool {
        assert!(self.exists(entity));
        self.try_path_component(entity).is_some()
    }

    pub fn vessel_component(&self, entity: Entity) -> &VesselComponent {
        assert!(self.exists(entity));
        self.model().vessel_components.get(entity)
    }

    pub fn try_vessel_component(&self, entity: Entity) -> Option<&VesselComponent> {
        assert!(self.exists(entity));
        self.model().vessel_components.try_get(entity)
    }

    pub fn has_vessel_component(&self, entity: Entity) -> bool {
        assert!(self.exists(entity));
        self.try_vessel_component(entity).is_some()
    }
}
