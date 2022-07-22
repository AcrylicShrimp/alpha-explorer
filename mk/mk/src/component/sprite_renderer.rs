use crate::render::{Color, Layer, LuaRcShader, LuaRcSprite, Shader, Sprite};
use codegen::{Animation, LuaComponent};
use std::sync::Arc;

#[derive(Animation, LuaComponent)]
pub struct SpriteRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    #[lua_userdata(LuaRcShader)]
    pub shader: Arc<Shader>,
    #[lua_userdata(LuaRcSprite)]
    pub sprite: Arc<Sprite>,
}

impl SpriteRenderer {
    pub fn new(shader: Arc<Shader>, sprite: Arc<Sprite>) -> Self {
        Self {
            layer: Layer::default(),
            order: 0,
            color: Color::white(),
            shader,
            sprite,
        }
    }
}
