use crate::render::{Color, Layer, LuaRcSpriteNinePatch, LuaShaderHandle};
use codegen::LuaComponent;

#[derive(LuaComponent, Debug)]
pub struct NinePatchRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: LuaShaderHandle,
    pub nine_patch: LuaRcSpriteNinePatch,
}

impl NinePatchRenderer {
    pub fn new(shader: LuaShaderHandle, nine_patch: LuaRcSpriteNinePatch) -> Self {
        Self {
            layer: Layer::default(),
            order: 0,
            color: Color::white(),
            shader,
            nine_patch,
        }
    }
}
