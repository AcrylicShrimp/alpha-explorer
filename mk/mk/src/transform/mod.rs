mod transform;
mod transform_flattener;
mod transform_manager;

pub use transform::*;
pub use transform_flattener::*;
pub use transform_manager::*;

#[cfg(test)]
pub mod test {
    use super::*;
    use std::intrinsics::transmute;
    use std::iter::FromIterator;

    #[test]
    fn test_transform_manager_alloc_dealloc() {
        let mut manager = TransformManager::default();
        let t0 = manager.alloc(unsafe { transmute(1u64) });
        let t1 = manager.alloc(unsafe { transmute(2u64) });
        let t2 = manager.alloc(unsafe { transmute(3u64) });
        let t3 = manager.alloc(unsafe { transmute(4u64) });
        let t4 = manager.alloc(unsafe { transmute(5u64) });
        let t5 = manager.alloc(unsafe { transmute(6u64) });
        let t6 = manager.alloc(unsafe { transmute(7u64) });
        let t7 = manager.alloc(unsafe { transmute(8u64) });
        let t8 = manager.alloc(unsafe { transmute(9u64) });
        let t9 = manager.alloc(unsafe { transmute(10u64) });

        assert_eq!(t0, 0);
        assert_eq!(t1, 1);
        assert_eq!(t2, 2);
        assert_eq!(t3, 3);
        assert_eq!(t4, 4);
        assert_eq!(t5, 5);
        assert_eq!(t6, 6);
        assert_eq!(t7, 7);
        assert_eq!(t8, 8);
        assert_eq!(t9, 9);

        manager.dealloc(t0);
        manager.dealloc(t1);
        manager.dealloc(t2);
        manager.dealloc(t3);
        manager.dealloc(t4);
        manager.dealloc(t5);
        manager.dealloc(t6);
        manager.dealloc(t7);
        manager.dealloc(t8);
        manager.dealloc(t9);

        let t0 = manager.alloc(unsafe { transmute(1u64) });
        let t1 = manager.alloc(unsafe { transmute(2u64) });
        let t2 = manager.alloc(unsafe { transmute(3u64) });
        let t3 = manager.alloc(unsafe { transmute(4u64) });
        let t4 = manager.alloc(unsafe { transmute(5u64) });
        let t5 = manager.alloc(unsafe { transmute(6u64) });
        let t6 = manager.alloc(unsafe { transmute(7u64) });
        let t7 = manager.alloc(unsafe { transmute(8u64) });
        let t8 = manager.alloc(unsafe { transmute(9u64) });
        let t9 = manager.alloc(unsafe { transmute(10u64) });

        assert_eq!(t0, 9);
        assert_eq!(t1, 8);
        assert_eq!(t2, 7);
        assert_eq!(t3, 6);
        assert_eq!(t4, 5);
        assert_eq!(t5, 4);
        assert_eq!(t6, 3);
        assert_eq!(t7, 2);
        assert_eq!(t8, 1);
        assert_eq!(t9, 0);
    }

    #[test]
    fn test_transform_manager_parent() {
        let mut manager = TransformManager::default();

        let parent = manager.alloc(unsafe { transmute(1u64) });
        let child0 = manager.alloc(unsafe { transmute(2u64) });
        let child1 = manager.alloc(unsafe { transmute(3u64) });
        let child2 = manager.alloc(unsafe { transmute(4u64) });

        manager.set_parent(child0, Some(parent));
        manager.set_parent(child1, Some(parent));
        manager.set_parent(child2, Some(parent));

        assert_eq!(
            {
                let mut children = Vec::from_iter(manager.children(parent).iter().copied());
                children.sort_unstable();
                children
            },
            &[child0, child1, child2]
        );
        assert_eq!(manager.transform(child0).parent_index(), Some(parent));
        assert_eq!(manager.transform(child1).parent_index(), Some(parent));
        assert_eq!(manager.transform(child2).parent_index(), Some(parent));

        manager.set_parent(child0, None);

        assert_eq!(
            {
                let mut children = Vec::from_iter(manager.children(parent).iter().copied());
                children.sort_unstable();
                children
            },
            &[child1, child2]
        );
        assert_eq!(manager.transform(child0).parent_index(), None);
        assert_eq!(manager.transform(child1).parent_index(), Some(parent));
        assert_eq!(manager.transform(child2).parent_index(), Some(parent));

        manager.set_parent(child1, None);

        assert_eq!(
            {
                let mut children = Vec::from_iter(manager.children(parent).iter().copied());
                children.sort_unstable();
                children
            },
            &[child2]
        );
        assert_eq!(manager.transform(child0).parent_index(), None);
        assert_eq!(manager.transform(child1).parent_index(), None);
        assert_eq!(manager.transform(child2).parent_index(), Some(parent));

        manager.set_parent(child2, None);

        assert_eq!(
            {
                let mut children = Vec::from_iter(manager.children(parent).iter().copied());
                children.sort_unstable();
                children
            },
            &[] as &[u32]
        );
        assert_eq!(manager.transform(child0).parent_index(), None);
        assert_eq!(manager.transform(child1).parent_index(), None);
        assert_eq!(manager.transform(child2).parent_index(), None);
    }
}
