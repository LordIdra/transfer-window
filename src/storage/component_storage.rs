use std::collections::HashSet;

use super::entity_allocator::Entity;

struct StorageEntry<T> {
    value: T,
    generation: usize,
}

pub struct ComponentStorage<T> {
    entities: HashSet<Entity>,
    entries: Vec<Option<StorageEntry<T>>>,
}

impl<T> ComponentStorage<T> {
    pub fn new() -> Self {
        Self { 
            entities: HashSet::new(),
            entries: vec![] 
        }
    }

    pub fn set(&mut self, entity: Entity, value: Option<T>) {
        if value.is_some() {
            self.entities.insert(entity);
        }
        let index = entity.get_index();
        let generation = entity.get_generation();
        let entry = value.map(|value| StorageEntry { value, generation });
        if let Some(current_entry) = self.entries.get_mut(index) {
            *current_entry = entry;
        } else if entity.get_index() == self.entries.len() {
            self.entries.push(entry);
        } else {
            panic!("Allocator and storages have desynced somewhere...")
        }
    }

    pub fn remove_if_exists(&mut self, entity: Entity) {
        let entry = self.entries.get_mut(entity.get_index());
        if let Some(entry) = entry {
            if let Some(entry) = entry {
                if entry.generation != entity.get_generation() {
                    panic!("Attempt to remove a component with an entity that has a different generation")
                }
                self.entities.remove(&entity);
            }
            *entry = None;
        }
    }

    pub fn get_mut(&mut self, entity: Entity) -> &mut T {
        let entry = self.entries
            .get_mut(entity.get_index())
            .expect("Entity index out of range")
            .as_mut();
        if let Some(entry) = entry {
            if entry.generation == entity.get_generation() {
                return &mut entry.value;
            }
        }
        panic!("Attempted to get nonexistant component");
    }

    pub fn get(&self, entity: Entity) -> &T {
        let entry = self.entries
            .get(entity.get_index())
            .expect("Entity index out of range");
        if let Some(entry) = entry {
            if entry.generation == entity.get_generation() {
                return &entry.value;
            }
        }
        panic!("Attempted to get nonexistant component");
    }

    pub fn get_entities(&self) -> &HashSet<Entity> {
        &self.entities
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::storage::entity_allocator::EntityAllocator;

    use super::ComponentStorage;

    #[test]
    fn test() {
        let mut allocator = EntityAllocator::new();
        let mut storage: ComponentStorage<f64> = ComponentStorage::new();
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
        let mut allocator = EntityAllocator::new();
        let mut storage: ComponentStorage<f64> = ComponentStorage::new();
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
        let mut allocator = EntityAllocator::new();
        let mut storage: ComponentStorage<f64> = ComponentStorage::new();
        let e1 = allocator.allocate();
        storage.remove_if_exists(e1);
        storage.remove_if_exists(e1);
    }

    #[test]
    fn test_entities() {
        // De-allocating an entity that was is not allocated should not cause any issues
        let mut allocator = EntityAllocator::new();
        let mut storage: ComponentStorage<f64> = ComponentStorage::new();
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
        assert!(*storage.get_entities() == expected);
    }
}