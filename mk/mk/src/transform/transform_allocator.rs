use crate::transform::Transform;
use specs::Entity;

#[derive(Debug)]
pub struct TransformAllocator {
    transforms: Vec<Transform>,
    entities: Vec<Entity>,
    removed_indices: Vec<u32>,
}

impl TransformAllocator {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn transform(&self, transform: u32) -> &Transform {
        &self.transforms[transform as usize]
    }

    pub fn transform_mut(&mut self, transform: u32) -> &mut Transform {
        &mut self.transforms[transform as usize]
    }

    pub fn entity(&self, transform: u32) -> Entity {
        self.entities[transform as usize]
    }

    pub fn alloc(&mut self) -> u32 {
        if let Some(index) = self.removed_indices.pop() {
            self.transforms[index as usize] = Transform::default();
            index
        } else {
            let index = self.transforms.len() as u32;
            self.transforms.push(Transform::default());
            index
        }
    }

    pub fn alloc_entity(&mut self, transform: u32, entity: Entity) {
        let transform_usize = transform as usize;

        if transform_usize < self.entities.len() {
            self.entities[transform_usize] = entity;
        } else {
            self.entities.push(entity);
        }
    }

    pub fn dealloc(&mut self, transform: u32) {
        self.removed_indices.push(transform);
    }
}

impl Default for TransformAllocator {
    fn default() -> Self {
        Self {
            transforms: Vec::with_capacity(1024),
            entities: Vec::with_capacity(1024),
            removed_indices: Vec::with_capacity(1024),
        }
    }
}
