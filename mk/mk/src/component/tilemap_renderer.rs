use crate::render::{Color, Layer, LuaRcTilemap, LuaShaderHandle};
use codegen::LuaComponent;

#[derive(LuaComponent, Debug)]
pub struct TilemapRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: LuaShaderHandle,
    pub tilemap: LuaRcTilemap,
}

impl TilemapRenderer {
    pub fn new(shader: LuaShaderHandle, tilemap: LuaRcTilemap) -> TilemapRenderer {
        TilemapRenderer {
            layer: Layer::default(),
            order: 0,
            color: Color::white(),
            shader,
            tilemap,
        }
    }
}
