use crate::render::{LuaRcSpriteAtlasGrid, SpriteAtlasGrid};
use codegen::LuaRc;
use std::sync::Arc;

#[derive(LuaRc, Debug)]
pub struct Tilemap {
    pub tile_width: f32,
    pub tile_height: f32,
    pub tile_count_x: usize,
    pub tile_count_y: usize,
    pub layers: Vec<Vec<usize>>,
    #[lua_userdata(LuaRcSpriteAtlasGrid)]
    pub palette: Arc<SpriteAtlasGrid>,
}
