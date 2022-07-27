use crate::render::LuaRcSpriteAtlasGrid;
use codegen::LuaRc;

#[derive(LuaRc)]
pub struct Tilemap {
    pub tile_width: f32,
    pub tile_height: f32,
    pub tile_count_x: usize,
    pub tile_count_y: usize,
    pub layers: Vec<Vec<usize>>,
    pub palette: LuaRcSpriteAtlasGrid,
}
