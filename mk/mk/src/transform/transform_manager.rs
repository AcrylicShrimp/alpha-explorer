use crate::{
    structure::{Mat33Mut, Mat33Ref},
    transform::{TransformAllocator, TransformHierarchy, TransformNameManager},
};
use specs::prelude::*;

#[derive(Debug)]
pub struct TransformManager {
    world_matrices: Vec<f32>,
    hierarchy: TransformHierarchy,
    allocator: TransformAllocator,
    name_manager: TransformNameManager,
}

impl TransformManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hierarchy(&self) -> &TransformHierarchy {
        &self.hierarchy
    }

    pub fn hierarchy_mut(&mut self) -> &mut TransformHierarchy {
        &mut self.hierarchy
    }

    pub fn allocator(&self) -> &TransformAllocator {
        &self.allocator
    }

    pub fn allocator_mut(&mut self) -> &mut TransformAllocator {
        &mut self.allocator
    }

    pub fn name_manager(&self) -> &TransformNameManager {
        &self.name_manager
    }

    pub fn name_manager_mut(&mut self) -> &mut TransformNameManager {
        &mut self.name_manager
    }

    pub fn alloc(&mut self) -> u32 {
        let transform = self.allocator.alloc();
        self.hierarchy.add(transform);
        self.name_manager.add(transform);

        let transform_usize = transform as usize;
        if self.world_matrices.len() <= transform_usize * 9 {
            self.world_matrices.extend_from_slice(&[0f32; 9]);
        }

        transform
    }

    pub fn alloc_entity(&mut self, transform: u32, entity: Entity) {
        self.allocator.alloc_entity(transform, entity);
    }

    pub fn dealloc(&mut self, transform: u32) {
        self.allocator.dealloc(transform);

        for &transform in self.hierarchy.transform_and_children(transform) {
            self.name_manager.remove_from_name_map(transform);
        }

        self.hierarchy.remove(transform);
    }

    pub fn find_children_by_name(
        &self,
        mut transform: u32,
        children: &[impl AsRef<str>],
    ) -> Option<u32> {
        'outer: for name in children {
            let name = name.as_ref();

            if let Some(children_iter) = self.hierarchy.direct_children_iter(transform) {
                for child in children_iter {
                    match self.name_manager.name(child) {
                        Some(child_name) => {
                            if child_name == name {
                                transform = child;
                                continue 'outer;
                            }
                        }
                        None => {}
                    }
                }
            }

            return None;
        }

        Some(transform)
    }

    pub fn transform_world_matrix(&self, index: u32) -> Mat33Ref {
        Mat33Ref::new(unsafe {
            &*(self.world_matrices.as_ptr().add(index as usize * 9) as *const [f32; 9])
        })
    }

    pub fn transform_world_matrix_mut(&self, index: u32) -> Mat33Mut {
        Mat33Mut::new(unsafe {
            &mut *(self.world_matrices.as_ptr().add(index as usize * 9) as *mut [f32; 9])
        })
    }

    pub fn update_world_matrices(&mut self) {
        for &transform in self.hierarchy.ordered_transforms() {
            if !self.hierarchy.is_dirty(transform) {
                continue;
            }

            let unfilled_mat = self.transform_world_matrix_mut(transform);
            let mut filled_mat = self
                .allocator
                .transform(transform)
                .fill_matrix(unfilled_mat);

            if let Some(parent) = self.hierarchy.parent(transform) {
                let parent_mat = self.transform_world_matrix_mut(parent);
                filled_mat *= parent_mat;
            }
        }

        self.hierarchy.reset_dirties()
    }
}

impl Default for TransformManager {
    fn default() -> Self {
        Self {
            world_matrices: Vec::with_capacity(1024),
            hierarchy: Default::default(),
            allocator: Default::default(),
            name_manager: Default::default(),
        }
    }
}
