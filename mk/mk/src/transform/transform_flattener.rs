use crate::transform::Transform;
use bitvec::prelude::*;

#[derive(Debug)]
pub struct TransformFlattener {
    processed_flags: BitVec,
    flattened_indices: Vec<usize>,
}

impl TransformFlattener {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self) {
        self.processed_flags.push(false);
        self.flattened_indices.push(0);
    }

    pub fn flatten(&mut self, transforms: &mut [Transform]) -> &[usize] {
        // 1. Mark all transforms as dirty that have a dirty parent transform.
        let range = transforms.as_mut_ptr_range();

        loop {
            let mut marked = false;
            let (mut ptr, end_ptr) = (range.start, range.end);

            while ptr != end_ptr {
                if !(unsafe { &mut *ptr }).is_dirty() {
                    if let Some(parent_index) = (unsafe { &mut *ptr }).parent_index() {
                        if transforms[parent_index as usize].is_dirty() {
                            marked = true;
                            (unsafe { &mut *ptr }).mark_as_dirty();
                        }
                    }
                }

                ptr = unsafe { ptr.add(1) };
            }

            if !marked {
                break;
            }
        }

        // 2. Clear processed_flags to all false.
        self.processed_flags.set_all(false);

        // 3. Clear the flattened_indices.
        self.flattened_indices.clear();

        // 4. Add transform indices that are ditry and its parent is processed(already in the flattened_indices).
        loop {
            let mut added = false;

            for (index, transform) in transforms.iter().enumerate() {
                if self.processed_flags[index] || !transform.is_dirty() {
                    continue;
                }

                if let Some(parent_index) = transform.parent_index() {
                    let parent_index = parent_index as usize;

                    if !transforms[parent_index].is_dirty() || self.processed_flags[parent_index] {
                        added = true;
                        self.processed_flags.set(index, true);
                        self.flattened_indices.push(index);
                    }
                } else {
                    added = true;
                    self.processed_flags.set(index, true);
                    self.flattened_indices.push(index);
                }
            }

            if !added {
                break;
            }
        }

        &mut self.flattened_indices
    }
}

impl Default for TransformFlattener {
    fn default() -> Self {
        Self {
            processed_flags: BitVec::with_capacity(1024),
            flattened_indices: Vec::with_capacity(1024),
        }
    }
}
