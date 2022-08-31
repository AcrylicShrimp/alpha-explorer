use crate::render::Layer;

pub struct Camera {
    pub layer: Layer,
    pub order: isize,
}

impl Camera {
    pub fn new(layer: Layer, order: isize) -> Self {
        Self { layer, order }
    }
}
