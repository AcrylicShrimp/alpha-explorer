use crate::render::{Color, Layer, Shader, Tilemap};
use std::sync::Arc;

pub struct TilemapRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub tilemap: Arc<Tilemap>,
}

impl TilemapRenderer {
    pub fn new(
        layer: Layer,
        order: isize,
        color: Color,
        shader: Arc<Shader>,
        tilemap: Arc<Tilemap>,
    ) -> Self {
        Self {
            layer,
            order,
            color,
            shader,
            tilemap,
        }
    }
}
