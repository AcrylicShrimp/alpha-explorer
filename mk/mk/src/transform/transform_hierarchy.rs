use std::{cmp::Ordering, ops::Range};

use bitvec::vec::BitVec;

#[derive(Debug, Clone, Copy, Eq, Ord, Hash)]
pub struct TransformSpan {
    pub index: u32,
    pub count: u32,
}

impl TransformSpan {
    pub fn to_range(self) -> Range<usize> {
        self.index as usize..(self.index + self.count) as usize
    }
}

impl PartialEq for TransformSpan {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl PartialOrd for TransformSpan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl From<TransformSpan> for Range<u32> {
    fn from(span: TransformSpan) -> Self {
        span.index..span.index + span.count
    }
}

impl From<TransformSpan> for Range<usize> {
    fn from(span: TransformSpan) -> Self {
        span.index as usize..(span.index + span.count) as usize
    }
}

#[derive(Debug, Clone)]
pub struct TransformSiblingIter<'a> {
    parent: u32,
    transform: u32,
    sibling_index: usize,
    transform_spans: &'a [TransformSpan],
    ordered_transforms: &'a [u32],
}

impl<'a> TransformSiblingIter<'a> {
    pub fn new(
        parent: Option<u32>,
        transform: u32,
        transform_spans: &'a [TransformSpan],
        ordered_transforms: &'a [u32],
    ) -> Self {
        Self {
            parent: parent.unwrap_or(transform),
            transform,
            sibling_index: 0,
            transform_spans,
            ordered_transforms,
        }
    }
}

impl<'a> Iterator for TransformSiblingIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.parent == self.transform {
            return if self.sibling_index == 0 {
                self.sibling_index += 1;
                Some(self.transform)
            } else {
                None
            };
        }

        let parent_span = self.transform_spans[self.parent as usize];
        let parent_span_index = parent_span.index as usize;
        let parent_span_count = parent_span.count as usize;
        let index = parent_span_index + 1 + self.sibling_index;

        if index < parent_span_index + parent_span_count {
            let transform = self.ordered_transforms[index];
            self.sibling_index += self.transform_spans[transform as usize].count as usize;
            Some(transform)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct TransformHierarchy {
    ordered_transforms: Vec<u32>,        // Ordered
    transform_spans: Vec<TransformSpan>, // Unordered
    transform_dirties: BitVec,           // Ordered
    transform_parents: Vec<Vec<u32>>,    // Unordered
}

impl TransformHierarchy {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_dirty(&self, transform: u32) -> bool {
        self.transform_dirties[self.transform_spans[transform as usize].index as usize]
    }

    pub fn parent(&self, transform: u32) -> Option<u32> {
        self.transform_parents[transform as usize].first().copied()
    }

    pub fn parents(&self, transform: u32) -> &[u32] {
        &self.transform_parents[transform as usize]
    }

    pub fn children(&self, transform: u32) -> &[u32] {
        let span = self.transform_spans[transform as usize];
        &self.ordered_transforms[(span.index + 1) as usize..(span.index + span.count) as usize]
    }

    pub fn direct_children_iter(&self, transform: u32) -> Option<TransformSiblingIter> {
        let span = self.transform_spans[transform as usize];
        if span.count < 2 {
            None
        } else {
            Some(TransformSiblingIter::new(
                Some(transform),
                self.ordered_transforms[span.index as usize + 1],
                &self.transform_spans,
                &self.ordered_transforms,
            ))
        }
    }

    pub fn transform_and_children(&self, transform: u32) -> &[u32] {
        let span = self.transform_spans[transform as usize];
        &self.ordered_transforms[span.index as usize..(span.index + span.count) as usize]
    }

    pub fn ordered_transforms(&self) -> &[u32] {
        &self.ordered_transforms
    }

    pub fn sibling_iter(&self, transform: u32) -> TransformSiblingIter {
        TransformSiblingIter::new(
            self.transform_parents[transform as usize].first().copied(),
            transform,
            &self.transform_spans,
            &self.ordered_transforms,
        )
    }

    pub fn set_dirty(&mut self, transform: u32) {
        self.transform_dirties.as_mut_bitslice()
            [self.transform_spans[transform as usize].to_range()]
        .fill(true);
    }

    pub fn reset_dirties(&mut self) {
        self.transform_dirties.fill(false);
    }

    pub fn add(&mut self, transform: u32) {
        let transform_usize = transform as usize;

        if transform_usize < self.transform_spans.len() {
            self.transform_spans[transform_usize] = TransformSpan {
                index: self.ordered_transforms.len() as u32,
                count: 1,
            };
            self.transform_parents[transform_usize].clear();
        } else {
            debug_assert!(transform_usize == self.transform_spans.len());
            self.transform_spans.push(TransformSpan {
                index: self.ordered_transforms.len() as u32,
                count: 1,
            });
            self.transform_parents.push(Vec::with_capacity(4));
        }

        self.ordered_transforms.push(transform);
        self.transform_dirties.push(true);
    }

    pub fn remove(&mut self, transform: u32) {
        let transform_usize = transform as usize;
        let span = self.transform_spans[transform_usize];

        // Remove the transform and its children from its parents.
        for &parent in &self.transform_parents[transform_usize] {
            let parent_usize = parent as usize;
            self.transform_spans[parent_usize].count -= span.count;
        }

        let span_index = span.index as usize;
        let span_count = span.count as usize;

        // Remove the transform and its children from the ordered transforms.
        for &transform in &self.ordered_transforms[span_index + span_count..] {
            self.transform_spans[transform as usize].index -= span.count;
        }

        if span_index + span_count < self.ordered_transforms.len() {
            self.ordered_transforms
                .copy_within(span_index + span_count.., span_index);
        }

        self.ordered_transforms
            .truncate(self.ordered_transforms.len() - span_count);

        if span_index + span_count < self.transform_dirties.len() {
            self.transform_dirties
                .copy_within(span_index + span_count.., span_index);
        }

        self.transform_dirties
            .truncate(self.transform_dirties.len() - span_count);
    }

    pub fn set_parent(&mut self, transform: u32, parent: Option<u32>) {
        self.set_dirty(transform);

        let transform_usize = transform as usize;
        let span = self.transform_spans[transform_usize];

        // Remove the transform and its children from its parents.
        for &parent in &self.transform_parents[transform_usize] {
            let parent_usize = parent as usize;
            self.transform_spans[parent_usize].count -= span.count;
        }

        let parent_count = self.transform_parents[transform_usize].len();

        // Remove the parents of the transform and its children.
        for &transform in &self.ordered_transforms[span.to_range()] {
            let parents = &mut self.transform_parents[transform as usize];
            parents.truncate(parents.len() - parent_count);
        }

        let destination_index = if let Some(parent) = parent {
            let parent_usize = parent as usize;
            let (left, right) = self.transform_parents.split_at_mut(parent_usize);
            let (high_parents, right) = right.split_first_mut().unwrap();

            // Assign a new parent and its parents.
            for &transform in &self.ordered_transforms[span.to_range()] {
                let parents = if transform < parent {
                    &mut left[transform as usize]
                } else {
                    &mut right[transform as usize - parent_usize - 1]
                };
                parents.reserve(high_parents.len() + 1);
                parents.push(parent);
                parents.extend_from_slice(high_parents);
            }

            let prev_parent_span = self.transform_spans[parent_usize];

            // Add the transform and its children to its new parent.
            self.transform_spans[parent_usize].count += span.count;

            for &high_parent in high_parents.iter() {
                let high_parent_usize = high_parent as usize;
                self.transform_spans[high_parent_usize].count += span.count;
            }

            (prev_parent_span.index + prev_parent_span.count) as usize
        } else {
            self.ordered_transforms.len()
        };

        // Move the transform and its children to the new destination.
        self.move_ordered_transforms(transform, destination_index);

        // Set dirties.
        self.set_dirty(transform);
    }

    /// Moves the given transform and its children to the destination index.
    fn move_ordered_transforms(&mut self, transform: u32, destination_index: usize) {
        let transform = transform as usize;
        let span = self.transform_spans[transform];
        let span_index = span.index as usize;
        let span_count = span.count as usize;
        let span_index_end = span_index + span_count;

        if destination_index == span_index {
            return;
        }

        if destination_index < span_index {
            let offset = (span_index - destination_index) as u32;

            for &transform in &self.ordered_transforms[span_index..span_index_end] {
                self.transform_spans[transform as usize].index -= offset;
            }

            for &transform in &self.ordered_transforms[destination_index..span_index] {
                self.transform_spans[transform as usize].index += span.count;
            }

            self.swap_range(destination_index, span_index, span_index_end);
        } else {
            let offset = (destination_index - span_index - span_count) as u32;

            for &transform in &self.ordered_transforms[span_index..span_index_end] {
                self.transform_spans[transform as usize].index += offset;
            }

            for &transform in &self.ordered_transforms[span_index_end..destination_index] {
                self.transform_spans[transform as usize].index -= span.count;
            }

            self.swap_range(span_index, span_index_end, destination_index);
        }
    }

    // Swaps the given two range index_left..index_mid and index_mid..index_right.
    fn swap_range(&mut self, index_left: usize, index_mid: usize, index_right: usize) {
        debug_assert!(index_left <= index_mid);
        debug_assert!(index_mid <= index_right);

        let (temp, temp_dest, src, dest) = if index_mid - index_left < index_right - index_mid {
            (
                index_left..index_mid,
                index_right - (index_mid - index_left),
                index_mid..index_right,
                index_left,
            )
        } else {
            (
                index_mid..index_right,
                index_left,
                index_left..index_mid,
                index_right - (index_mid - index_left),
            )
        };

        let temp_ordered_transforms = self.ordered_transforms[temp.clone()].to_vec();
        self.ordered_transforms.copy_within(src.clone(), dest);
        self.ordered_transforms[temp_dest..temp_dest + temp.len()]
            .copy_from_slice(&temp_ordered_transforms);

        let temp_transform_dirties = self.transform_dirties[temp.clone()].to_bitvec();
        self.transform_dirties.copy_within(src, dest);
        self.transform_dirties[temp_dest..temp_dest + temp.len()]
            .copy_from_bitslice(&temp_transform_dirties);
    }
}

impl Default for TransformHierarchy {
    fn default() -> Self {
        Self {
            ordered_transforms: Vec::with_capacity(1024),
            transform_spans: Vec::with_capacity(1024),
            transform_dirties: BitVec::with_capacity(1024),
            transform_parents: Vec::with_capacity(1024),
        }
    }
}
