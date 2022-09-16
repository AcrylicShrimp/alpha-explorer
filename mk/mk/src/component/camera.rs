use crate::render::Layer;
use specs::{prelude::*, Component};

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Camera {
    pub layer: Layer,
    pub order: isize,
}

impl Camera {
    pub fn new(layer: Layer, order: isize) -> Self {
        Self { layer, order }
    }
}
