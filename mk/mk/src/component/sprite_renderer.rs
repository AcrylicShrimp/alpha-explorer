use crate::render::{Color, Layer, Shader, Sprite};
use specs::{prelude::*, Component};
use std::sync::Arc;

#[derive(Component)]
pub struct SpriteRenderer {
    pub layer: Layer,
    pub order: i32,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub sprite: Arc<Sprite>,
}

impl SpriteRenderer {
    pub fn new(
        layer: Layer,
        order: i32,
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
