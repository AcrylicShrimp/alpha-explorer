mod transform;
mod transform_allocator;
mod transform_hierarchy;
mod transform_manager;
mod transform_name_manager;

pub use transform::*;
pub use transform_allocator::*;
pub use transform_hierarchy::*;
pub use transform_manager::*;
pub use transform_name_manager::*;

#[cfg(test)]
pub mod test {
    use super::*;
    use std::intrinsics::transmute;
    use std::iter::FromIterator;

    #[test]
    fn test_transform_hierarchy() {
        let mut hierarchy = TransformHierarchy::default();

        hierarchy.add(0);
        hierarchy.add(1);
        hierarchy.add(2);
        hierarchy.add(3);
        hierarchy.add(4);
        hierarchy.add(5);
        hierarchy.add(6);
        hierarchy.add(7);
        hierarchy.add(8);
        hierarchy.add(9);
        hierarchy.add(10);
        hierarchy.add(11);
        hierarchy.add(12);
        hierarchy.add(13);
        hierarchy.add(14);
        hierarchy.add(15);
        hierarchy.add(16);
        hierarchy.add(17);
        hierarchy.add(18);
        hierarchy.add(19);
        hierarchy.add(20);
        assert_eq!(
            hierarchy.ordered_transforms(),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
        );

        hierarchy.set_parent(15, Some(1));
        hierarchy.set_parent(16, Some(1));
        hierarchy.set_parent(17, Some(1));
        hierarchy.set_parent(18, Some(1));
        hierarchy.set_parent(19, Some(1));
        hierarchy.set_parent(20, Some(1));
        assert_eq!(
            hierarchy.ordered_transforms(),
            &[0, 1, 15, 16, 17, 18, 19, 20, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
        );
        assert_eq!(hierarchy.parents(15), &[1]);
        assert_eq!(hierarchy.parents(16), &[1]);
        assert_eq!(hierarchy.parents(17), &[1]);
        assert_eq!(hierarchy.parents(18), &[1]);
        assert_eq!(hierarchy.parents(19), &[1]);
        assert_eq!(hierarchy.parents(20), &[1]);
        assert_eq!(hierarchy.children(1), &[15, 16, 17, 18, 19, 20]);

        hierarchy.set_parent(1, Some(10));
        assert_eq!(
            hierarchy.ordered_transforms(),
            &[0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 15, 16, 17, 18, 19, 20, 11, 12, 13, 14],
        );
        assert_eq!(hierarchy.parents(1), &[10]);
        assert_eq!(hierarchy.parents(15), &[1, 10]);
        assert_eq!(hierarchy.parents(16), &[1, 10]);
        assert_eq!(hierarchy.parents(17), &[1, 10]);
        assert_eq!(hierarchy.parents(18), &[1, 10]);
        assert_eq!(hierarchy.parents(19), &[1, 10]);
        assert_eq!(hierarchy.parents(20), &[1, 10]);
        assert_eq!(hierarchy.children(10), &[1, 15, 16, 17, 18, 19, 20]);

        hierarchy.set_parent(2, Some(10));
        hierarchy.set_parent(3, Some(10));
        hierarchy.set_parent(4, Some(10));
        hierarchy.set_parent(5, Some(10));
        assert_eq!(
            hierarchy.ordered_transforms(),
            &[0, 6, 7, 8, 9, 10, 1, 15, 16, 17, 18, 19, 20, 2, 3, 4, 5, 11, 12, 13, 14],
        );
        assert_eq!(hierarchy.parents(2), &[10]);
        assert_eq!(hierarchy.parents(3), &[10]);
        assert_eq!(hierarchy.parents(4), &[10]);
        assert_eq!(hierarchy.parents(5), &[10]);
        assert_eq!(
            hierarchy.children(10),
            &[1, 15, 16, 17, 18, 19, 20, 2, 3, 4, 5]
        );

        hierarchy.set_parent(6, Some(15));
        hierarchy.set_parent(7, Some(15));
        hierarchy.set_parent(8, Some(15));
        hierarchy.set_parent(9, Some(15));
        assert_eq!(
            hierarchy.ordered_transforms(),
            &[0, 10, 1, 15, 6, 7, 8, 9, 16, 17, 18, 19, 20, 2, 3, 4, 5, 11, 12, 13, 14],
        );
        assert_eq!(hierarchy.parents(6), &[15, 1, 10]);
        assert_eq!(hierarchy.parents(7), &[15, 1, 10]);
        assert_eq!(hierarchy.parents(8), &[15, 1, 10]);
        assert_eq!(hierarchy.parents(9), &[15, 1, 10]);
        assert_eq!(hierarchy.children(15), &[6, 7, 8, 9]);
        assert_eq!(hierarchy.children(1), &[15, 6, 7, 8, 9, 16, 17, 18, 19, 20]);
        assert_eq!(
            hierarchy.children(10),
            &[1, 15, 6, 7, 8, 9, 16, 17, 18, 19, 20, 2, 3, 4, 5]
        );

        hierarchy.set_parent(11, Some(9));
        hierarchy.set_parent(12, Some(9));
        hierarchy.set_parent(13, Some(9));
        hierarchy.set_parent(14, Some(9));
        assert_eq!(
            hierarchy.ordered_transforms(),
            &[0, 10, 1, 15, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 20, 2, 3, 4, 5,],
        );
        assert_eq!(hierarchy.parents(11), &[9, 15, 1, 10]);
        assert_eq!(hierarchy.parents(12), &[9, 15, 1, 10]);
        assert_eq!(hierarchy.parents(13), &[9, 15, 1, 10]);
        assert_eq!(hierarchy.parents(14), &[9, 15, 1, 10]);
        assert_eq!(hierarchy.children(9), &[11, 12, 13, 14]);
        assert_eq!(hierarchy.children(15), &[6, 7, 8, 9, 11, 12, 13, 14]);
        assert_eq!(
            hierarchy.children(1),
            &[15, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 20]
        );
        assert_eq!(
            hierarchy.children(10),
            &[1, 15, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 20, 2, 3, 4, 5]
        );

        assert_eq!(hierarchy.sibling_iter(0).collect::<Vec<_>>(), &[0]);
        assert_eq!(hierarchy.sibling_iter(10).collect::<Vec<_>>(), &[10]);
        assert_eq!(
            hierarchy.sibling_iter(1).collect::<Vec<_>>(),
            &[1, 2, 3, 4, 5]
        );
        assert_eq!(
            hierarchy.sibling_iter(15).collect::<Vec<_>>(),
            &[15, 16, 17, 18, 19, 20]
        );
        assert_eq!(
            hierarchy.sibling_iter(16).collect::<Vec<_>>(),
            &[15, 16, 17, 18, 19, 20]
        );
        assert_eq!(
            hierarchy.sibling_iter(17).collect::<Vec<_>>(),
            &[15, 16, 17, 18, 19, 20]
        );
        assert_eq!(
            hierarchy.sibling_iter(18).collect::<Vec<_>>(),
            &[15, 16, 17, 18, 19, 20]
        );
        assert_eq!(
            hierarchy.sibling_iter(19).collect::<Vec<_>>(),
            &[15, 16, 17, 18, 19, 20]
        );
        assert_eq!(
            hierarchy.sibling_iter(20).collect::<Vec<_>>(),
            &[15, 16, 17, 18, 19, 20]
        );
        assert_eq!(
            hierarchy.sibling_iter(11).collect::<Vec<_>>(),
            &[11, 12, 13, 14]
        );
        assert_eq!(
            hierarchy.sibling_iter(12).collect::<Vec<_>>(),
            &[11, 12, 13, 14]
        );
        assert_eq!(
            hierarchy.sibling_iter(13).collect::<Vec<_>>(),
            &[11, 12, 13, 14]
        );
        assert_eq!(
            hierarchy.sibling_iter(14).collect::<Vec<_>>(),
            &[11, 12, 13, 14]
        );

        hierarchy.set_parent(1, None);
        assert_eq!(
            hierarchy.ordered_transforms(),
            &[0, 10, 2, 3, 4, 5, 1, 15, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 20,],
        );
        assert_eq!(hierarchy.parents(1).len(), 0);
        assert_eq!(hierarchy.parents(15), &[1]);
        assert_eq!(hierarchy.parents(6), &[15, 1]);
        assert_eq!(hierarchy.parents(7), &[15, 1]);
        assert_eq!(hierarchy.parents(8), &[15, 1]);
        assert_eq!(hierarchy.parents(9), &[15, 1]);
        assert_eq!(hierarchy.parents(11), &[9, 15, 1]);
        assert_eq!(hierarchy.parents(12), &[9, 15, 1]);
        assert_eq!(hierarchy.parents(13), &[9, 15, 1]);
        assert_eq!(hierarchy.parents(14), &[9, 15, 1]);
        assert_eq!(
            hierarchy.children(1),
            &[15, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 20]
        );
        assert_eq!(hierarchy.children(10), &[2, 3, 4, 5,]);

        hierarchy.remove(10);
        assert_eq!(
            hierarchy.ordered_transforms(),
            &[0, 1, 15, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 20,],
        );

        hierarchy.remove(9);
        assert_eq!(
            hierarchy.ordered_transforms(),
            &[0, 1, 15, 6, 7, 8, 16, 17, 18, 19, 20,],
        );
        assert_eq!(hierarchy.children(1), &[15, 6, 7, 8, 16, 17, 18, 19, 20,]);

        hierarchy.remove(1);
        assert_eq!(hierarchy.ordered_transforms(), &[0,],);
    }
}
