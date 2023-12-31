use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Entity {
    index: usize,
    generation: usize,
}

impl Entity {
    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_generation(&self) -> usize {
        self.generation
    }
}

struct AllocatorEntry {
    is_allocated: bool,
    generation: usize,
}

pub struct EntityAllocator {
    entities: HashSet<Entity>,
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}

impl EntityAllocator {
    pub fn new() -> Self {
        Self { entities: HashSet::new(), entries: vec![], free: vec![] }
    }

    pub fn allocate(&mut self) -> Entity {
        if let Some(index) = self.free.pop() {
            if self.entries[index].is_allocated {
                panic!("Attempt to allocate to an index that was already allocated");
            }
            self.entries[index].is_allocated = true;
            let generation = self.entries[index].generation;
            let entity = Entity { index, generation };
            self.entities.insert(entity);
            return entity;
        }
        let index = self.entries.len();
        let is_allocated = true;
        let generation = 0;
        self.entries.push(AllocatorEntry { is_allocated, generation });
        let entity = Entity { index, generation };
        self.entities.insert(entity);
        entity
    }

    pub fn deallocate(&mut self, entity: Entity) {
        if !self.entries[entity.index].is_allocated {
            panic!("Attempt to deallocate an entity that was already deallocated");
        }
        self.entries[entity.index].is_allocated = false;
        self.entries[entity.index].generation += 1;
        self.entities.remove(&entity);
        self.free.push(entity.index);
    }

    pub fn get_entities(&self) -> HashSet<Entity> {
        self.entities.clone()
    }
}
