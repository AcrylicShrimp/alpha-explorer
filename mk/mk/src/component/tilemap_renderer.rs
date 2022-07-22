use crate::render::{Color, Layer, LuaRcShader, LuaRcTilemap, Shader, Tilemap};
use codegen::LuaComponent;
use std::sync::Arc;

#[derive(LuaComponent, Debug)]
pub struct TilemapRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    #[lua_userdata(LuaRcShader)]
    pub shader: Arc<Shader>,
    #[lua_userdata(LuaRcTilemap)]
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
