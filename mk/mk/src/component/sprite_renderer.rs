use crate::render::{Color, Layer, Shader, Sprite};
use std::sync::Arc;

pub struct SpriteRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub sprite: Arc<Sprite>,
}

impl SpriteRenderer {
    pub fn new(
        layer: Layer,
        order: isize,
        color: Color,
        shader: Arc<Shader>,
        sprite: Arc<Sprite>,
    ) -> Self {
        Self {
            layer,
            order,
            color,
            shader,
            sprite,
        }
    }
}
