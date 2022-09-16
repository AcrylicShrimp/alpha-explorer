use crate::engine::use_context;
use specs::{prelude::*, Component};

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
#[storage(VecStorage)]
pub struct Transform {
    index: u32,
}

impl Transform {
    pub fn new(index: u32) -> Self {
        Self { index }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn with_ref<T>(&self, f: impl FnOnce(&crate::transform::Transform) -> T) -> T {
        let transform_mgr = use_context().transform_mgr();
        f(transform_mgr.transform(self.index))
    }

    pub fn with_mut<T>(&self, f: impl FnOnce(&mut crate::transform::Transform) -> T) -> T {
        let mut transform_mgr = use_context().transform_mgr_mut();
        f(transform_mgr.transform_mut(self.index))
    }
}
