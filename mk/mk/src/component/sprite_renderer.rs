use crate::render::{Color, Layer, LuaRcSprite, LuaShaderHandle};
use codegen::{Animation, LuaComponent};

#[derive(Animation, LuaComponent)]
pub struct SpriteRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: LuaShaderHandle,
    pub sprite: LuaRcSprite,
}

impl SpriteRenderer {
    pub fn new(shader: LuaShaderHandle, sprite: LuaRcSprite) -> Self {
        Self {
            layer: Layer::default(),
            order: 0,
            color: Color::white(),
            shader,
            sprite,
        }
    }
}
