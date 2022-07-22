use crate::transform::{Transform, TransformFlattener};
use legion::Entity;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TransformManager {
    transforms: Vec<Transform>,
    entities: Vec<Entity>,
    childrens: Vec<Vec<u32>>,
    names: Vec<Option<String>>,
    name_map: HashMap<String, Vec<u32>>,
    world_matrices: Vec<f32>,
    flattener: TransformFlattener,
    removed_indices: Vec<u32>,
}

impl TransformManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc(&mut self, entity: Entity) -> u32 {
        if let Some(index) = self.removed_indices.pop() {
            self.transforms[index as usize] = Transform::default();
            self.entities[index as usize] = entity;
            self.childrens[index as usize].clear();
            self.names[index as usize] = None;
            return index;
        }

        let index = self.transforms.len() as u32;
        self.transforms.push(Transform::default());
        self.entities.push(entity);
        self.childrens.push(Vec::new());
        self.names.push(None);
        self.world_matrices.extend_from_slice(&[0f32; 9]);
        self.flattener.push();
        index
    }

    pub fn dealloc(&mut self, index: u32) {
        if let Some(name) = &self.names[index as usize] {
            if let Some(names) = self.name_map.get_mut(name) {
                if let Some(index) = names.iter().position(|&transform| transform == index) {
                    names.swap_remove(index);
                }
            }
        }

        self.transforms[index as usize].reset_flags();
        self.removed_indices.push(index);
    }

    pub fn set_parent(&mut self, index: u32, parent_index: Option<u32>) {
        if let Some(parent_index) = self.transforms[index as usize].parent_index() {
            if let Some(index) = self.childrens[parent_index as usize]
                .iter()
                .position(|&child| child == index)
            {
                self.childrens[parent_index as usize].swap_remove(index);
            }
        }

        self.transforms[index as usize].set_parent_index(parent_index);

        if let Some(parent_index) = parent_index {
            self.childrens[parent_index as usize].push(index);
        }

        let mut current_index = index as usize;

        while let Some(parent_index) = self.transforms[current_index].parent_index() {
            if parent_index == index {
                if let Some(index) = self.childrens[parent_index as usize]
                    .iter()
                    .position(|&child| child as usize == current_index)
                {
                    self.childrens[parent_index as usize].swap_remove(index);
                }

                self.transforms[current_index].set_parent_index(None);
                return;
            }

            current_index = parent_index as usize;
        }
    }

    pub fn set_name(&mut self, index: u32, name: Option<String>) {
        if let Some(name) = &self.names[index as usize] {
            if let Some(indices) = self.name_map.get_mut(name) {
                if let Some(index) = indices.iter().position(|&transform| transform == index) {
                    indices.swap_remove(index);
                }
            }
        }

        if let Some(name) = name.clone() {
            self.name_map
                .entry(name)
                .or_insert_with(Vec::new)
                .push(index);
        }

        self.names[index as usize] = name;
    }

    pub fn transform(&self, index: u32) -> &Transform {
        &self.transforms[index as usize]
    }

    pub fn transform_mut(&mut self, index: u32) -> &mut Transform {
        &mut self.transforms[index as usize]
    }

    pub fn entity(&self, index: u32) -> Entity {
        self.entities[index as usize]
    }

    pub fn children(&self, index: u32) -> &[u32] {
        &self.childrens[index as usize]
    }

    pub fn name(&self, index: u32) -> Option<&str> {
        self.names[index as usize]
            .as_ref()
            .map(|name| name.as_str())
    }

    pub fn find_by_name(&self, name: impl AsRef<str>) -> Option<&[u32]> {
        self.name_map
            .get(name.as_ref())
            .map(|indices| indices.as_slice())
    }

    pub fn find_child_by_name(
        &self,
        mut index: u32,
        child: Option<&[impl AsRef<str>]>,
    ) -> Option<u32> {
        match child {
            Some(child) => {
                'outer: for name in child {
                    let name = name.as_ref();

                    for &child in self.children(index) {
                        match self.name(child) {
                            Some(child_name) => {
                                if child_name == name {
                                    index = child;
                                    continue 'outer;
                                }
                            }
                            None => {}
                        }
                    }

                    return None;
                }

                Some(index)
            }
            None => Some(index),
        }
    }

    pub fn transform_world_matrix(&self, index: u32) -> &[f32; 9] {
        unsafe {
            &*(self.world_matrices.as_ptr().add(index as usize * 9) as *const _ as *const [f32; 9])
        }
    }

    pub fn update_world_matrices(&mut self) {
        for &index in self.flattener.flatten(&mut self.transforms) {
            if let Some(parent_index) = self.transforms[index].parent_index() {
                let parent_index = parent_index as usize;
                let parent_matrix = unsafe {
                    &*(self.world_matrices.as_ptr().add(parent_index * 9) as *const _
                        as *const [f32; 9])
                };
                let matrix = unsafe {
                    &mut *(self.world_matrices.as_mut_ptr().add(index * 9) as *mut _
                        as *mut [f32; 9])
                };

                self.transforms[index].to_matrix(matrix);

                let m0 = matrix[0] * parent_matrix[0]
                    + matrix[1] * parent_matrix[3]
                    + matrix[2] * parent_matrix[6];
                let m1 = matrix[0] * parent_matrix[1]
                    + matrix[1] * parent_matrix[4]
                    + matrix[2] * parent_matrix[7];
                let m2 = matrix[0] * parent_matrix[2]
                    + matrix[1] * parent_matrix[5]
                    + matrix[2] * parent_matrix[8];
                let m3 = matrix[3] * parent_matrix[0]
                    + matrix[4] * parent_matrix[3]
                    + matrix[5] * parent_matrix[6];
                let m4 = matrix[3] * parent_matrix[1]
                    + matrix[4] * parent_matrix[4]
                    + matrix[5] * parent_matrix[7];
                let m5 = matrix[3] * parent_matrix[2]
                    + matrix[4] * parent_matrix[5]
                    + matrix[5] * parent_matrix[8];
                let m6 = matrix[6] * parent_matrix[0]
                    + matrix[7] * parent_matrix[3]
                    + matrix[8] * parent_matrix[6];
                let m7 = matrix[6] * parent_matrix[1]
                    + matrix[7] * parent_matrix[4]
                    + matrix[8] * parent_matrix[7];
                let m8 = matrix[6] * parent_matrix[2]
                    + matrix[7] * parent_matrix[5]
                    + matrix[8] * parent_matrix[8];

                matrix[0] = m0;
                matrix[1] = m1;
                matrix[2] = m2;
                matrix[3] = m3;
                matrix[4] = m4;
                matrix[5] = m5;
                matrix[6] = m6;
                matrix[7] = m7;
                matrix[8] = m8;
            } else {
                let matrix = unsafe {
                    &mut *(self.world_matrices.as_mut_ptr().add(index * 9) as *mut _
                        as *mut [f32; 9])
                };

                self.transforms[index].to_matrix(matrix);
            }

            self.transforms[index].reset_dirty();
        }
    }
}

impl Default for TransformManager {
    fn default() -> Self {
        Self {
            transforms: Vec::with_capacity(1024),
            entities: Vec::with_capacity(1024),
            childrens: Vec::with_capacity(1024),
            names: Vec::with_capacity(1024),
            name_map: HashMap::new(),
            world_matrices: Vec::with_capacity(1024 * 9),
            flattener: TransformFlattener::new(),
            removed_indices: Vec::with_capacity(1024),
        }
    }
}
