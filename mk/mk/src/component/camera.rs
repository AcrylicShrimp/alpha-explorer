use crate::gfx::{ClearMode, Color, Layer};
use specs::{prelude::*, Component};

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Camera {
    pub layer: Layer,
    pub order: isize,
    pub clear_mode: ClearMode,
    pub clear_color: Color,
}

impl Camera {
    pub fn new(layer: Layer, order: isize, clear_mode: ClearMode, clear_color: Color) -> Self {
        Self {
            layer,
            order,
            clear_mode,
            clear_color,
        }
    }
}
