use specs::{prelude::*, Component};

#[derive(Component)]
#[storage(VecStorage)]
#[derive(Clone, Copy)]
pub struct Size {
    index: u32,
    pub size: crate::structure::Size,
}

impl Size {
    pub fn new(index: u32) -> Self {
        Self {
            index,
            size: crate::structure::Size::zero(),
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}
