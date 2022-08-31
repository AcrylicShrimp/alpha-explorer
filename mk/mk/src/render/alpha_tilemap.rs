use crate::render::AlphaTileset;
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone)]
pub struct AlphaTilemap {
    pub tile_width: f32,
    pub tile_height: f32,
    pub tile_count_x: usize,
    pub tile_count_y: usize,
    pub layer: Vec<usize>,
    pub tileset: Arc<AlphaTileset>,
}

impl AlphaTilemap {
    pub fn new(
        tile_width: f32,
        tile_height: f32,
        tile_count_x: usize,
        tile_count_y: usize,
        layer: Vec<usize>,
        tileset: Arc<AlphaTileset>,
    ) -> Self {
        Self {
            tile_width,
            tile_height,
            tile_count_x,
            tile_count_y,
            layer,
            tileset,
        }
    }
}

impl Display for AlphaTilemap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AlphaTilemap({}x{}, tiles={})",
            self.tile_count_x,
            self.tile_count_y,
            self.tileset.tiles.len()
        )
    }
}
