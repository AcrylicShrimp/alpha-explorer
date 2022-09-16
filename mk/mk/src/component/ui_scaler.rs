use crate::{structure::Size, ui::UIScaleMode};
use specs::{prelude::*, Component};

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct UIScaler {
    pub mode: UIScaleMode,
    pub reference_size: Size,
}

impl UIScaler {
    pub fn new(mode: UIScaleMode, reference_size: Size) -> Self {
        Self {
            mode,
            reference_size,
        }
    }
}
