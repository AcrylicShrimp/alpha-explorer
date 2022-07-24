use crate::render::{Color, Layer, LuaRcTilemap, LuaShaderHandle, Shader, Tilemap};
use codegen::LuaComponent;
use std::sync::Arc;

#[derive(LuaComponent, Debug)]
pub struct TilemapRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    #[lua_user_type(LuaShaderHandle)]
    pub shader: Arc<Shader>,
    #[lua_user_type(LuaRcTilemap)]
    pub tilemap: Arc<Tilemap>,
}

impl TilemapRenderer {
    pub fn new(shader: Arc<Shader>, tilemap: Arc<Tilemap>) -> TilemapRenderer {
        TilemapRenderer {
            layer: Layer::default(),
            order: 0,
            color: Color::white(),
            shader,
            tilemap,
        }
    }
}
