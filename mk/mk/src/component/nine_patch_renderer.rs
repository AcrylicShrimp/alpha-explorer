use crate::render::{Color, Layer, Shader, SpriteNinePatch};
use std::sync::Arc;

pub struct NinePatchRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub nine_patch: Arc<SpriteNinePatch>,
}

impl NinePatchRenderer {
    pub fn new(
        layer: Layer,
        order: isize,
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
