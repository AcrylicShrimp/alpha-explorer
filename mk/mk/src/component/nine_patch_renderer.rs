use crate::render::{Color, Layer, Shader, SpriteNinePatch};
use specs::{prelude::*, Component};
use std::sync::Arc;

#[derive(Component)]
pub struct NinePatchRenderer {
    pub layer: Layer,
    pub order: i32,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub nine_patch: Arc<SpriteNinePatch>,
}

impl NinePatchRenderer {
    pub fn new(
        layer: Layer,
        order: i32,
        color: Color,
        shader: Arc<Shader>,
        nine_patch: Arc<SpriteNinePatch>,
    ) -> Self {
        Self {
            layer,
            order,
            color,
            shader,
            nine_patch,
        }
    }
}
