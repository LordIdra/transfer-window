use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::entity_allocator::Entity;

#[derive(Debug, Serialize, Deserialize)]
struct StorageEntry<T> {
    value: T,
    generation: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentStorage<T> {
    entities: HashSet<Entity>,
    entries: Vec<Option<StorageEntry<T>>>,
}

impl<T> Default for ComponentStorage<T> {
    fn default() -> Self {
        Self {
            entities: HashSet::new(),
            entries: vec![],
        }
    }
}

impl<T> ComponentStorage<T> {
    #[allow(clippy::missing_panics_doc)]
    pub fn set(&mut self, entity: Entity, value: Option<T>) {
        if value.is_some() {
            self.entities.insert(entity);
        }
        let index = entity.index();
        let generation = entity.generation();
        let entry = value.map(|value| StorageEntry { value, generation });
        if let Some(current_entry) = self.entries.get_mut(index) {
            *current_entry = entry;
        } else if entity.index() == self.entries.len() {
            self.entries.push(entry);
        } else {
            // This should never happen
            panic!("Detected allocator and storage desync");
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn remove_if_exists(&mut self, entity: Entity) {
        let entry = self.entries.get_mut(entity.index());
        if let Some(entry) = entry {
            if let Some(entry) = entry {
                assert!(
                    entry.generation == entity.generation(),
                    "Attempt to remove a component with an entity that has a different generation"
                );
                self.entities.remove(&entity);
            }
            *entry = None;
        }
    }

    /// # Panics
    /// Panics if the entity does not have an associated component
    pub fn get_mut(&mut self, entity: Entity) -> &mut T {
        if let Some(t) = self.try_get_mut(entity) {
            return t;
        }
        panic!("Attempted to get nonexistant component");
    }

    /// # Panics
    /// Panics if the entity does not have an associated component
    pub fn get(&self, entity: Entity) -> &T {
        if let Some(t) = self.try_get(entity) {
            return t;
        }
        panic!("Attempted to get nonexistant component");
    }

    pub fn try_get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let entry = self.entries[entity.index()].as_mut();
        if let Some(entry) = entry {
            if entry.generation == entity.generation() {
                return Some(&mut entry.value);
            }
        }
        None
    }

    pub fn try_get(&self, entity: Entity) -> Option<&T> {
        let entry = &self.entries[entity.index()];
        if let Some(entry) = entry {
            if entry.generation == entity.generation() {
                return Some(&entry.value);
            }
        }
        None
    }

    pub fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::ComponentStorage;
    use crate::storage::entity_allocator::EntityAllocator;

    #[test]
    fn test() {
        let mut allocator = EntityAllocator::default();
        let mut storage: ComponentStorage<f64> = ComponentStorage::default();
        let e1 = allocator.allocate();
        let e2 = allocator.allocate();
        storage.set(e1, Some(1.0));
        storage.set(e2, Some(6.0));
        assert!(*storage.get(e1) == 1.0);
        assert!(*storage.get(e2) == 6.0);
        *storage.get_mut(e1) += 1.0;
        assert!(*storage.get(e1) == 2.0);
    }

    #[test]
    #[should_panic]
    fn test_deallocate() {
        let mut allocator = EntityAllocator::default();
        let mut storage: ComponentStorage<f64> = ComponentStorage::default();
        let e1 = allocator.allocate();
        let e2 = allocator.allocate();
        storage.set(e1, Some(1.0));
        storage.set(e2, Some(6.0));
        storage.remove_if_exists(e1);
        storage.get(e1);
    }

    #[test]
    fn test_double_deallocate() {
        // De-allocating an entity that was is not allocated should not cause any issues
        let mut allocator = EntityAllocator::default();
        let mut storage: ComponentStorage<f64> = ComponentStorage::default();
        let e1 = allocator.allocate();
        storage.remove_if_exists(e1);
        storage.remove_if_exists(e1);
    }

    #[test]
    fn test_entities() {
        // De-allocating an entity that was is not allocated should not cause any issues
        let mut allocator = EntityAllocator::default();
        let mut storage: ComponentStorage<f64> = ComponentStorage::default();
        let e1 = allocator.allocate();
        let e2 = allocator.allocate();
        let e3 = allocator.allocate();
        storage.set(e1, Some(1.0));
        storage.set(e2, Some(1.0));
        storage.set(e3, Some(1.0));
        storage.remove_if_exists(e2);
        let mut expected = HashSet::new();
        expected.insert(e1);
        expected.insert(e3);
        assert!(*storage.entities() == expected);
    }
}
