use crate::render::{Color, Layer, LuaRcSpriteNinePatch, LuaShaderHandle, Shader, SpriteNinePatch};
use codegen::{Animation, LuaComponent};
use std::sync::Arc;

#[derive(Animation, LuaComponent)]
pub struct NinePatchRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    #[lua_user_type(LuaShaderHandle)]
    pub shader: Arc<Shader>,
    #[lua_user_type(LuaRcSpriteNinePatch)]
    pub nine_patch: Arc<SpriteNinePatch>,
}

impl NinePatchRenderer {
    pub fn new(shader: Arc<Shader>, nine_patch: Arc<SpriteNinePatch>) -> Self {
        Self {
            layer: Layer::default(),
            order: 0,
            color: Color::white(),
            shader,
            nine_patch,
        }
    }
}
