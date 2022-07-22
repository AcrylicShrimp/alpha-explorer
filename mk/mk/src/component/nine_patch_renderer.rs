use crate::render::{Color, Layer, LuaRcShader, LuaRcSpriteNinePatch, Shader, SpriteNinePatch};
use codegen::{Animation, LuaComponent};
use std::sync::Arc;

#[derive(Animation, LuaComponent)]
pub struct NinePatchRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    #[lua_userdata(LuaRcShader)]
    pub shader: Arc<Shader>,
    #[lua_userdata(LuaRcSpriteNinePatch)]
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
