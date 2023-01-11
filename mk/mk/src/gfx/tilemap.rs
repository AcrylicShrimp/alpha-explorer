use super::Sprite;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Tilemap {
    pub tile_width: f32,
    pub tile_height: f32,
    pub tile_count_x: usize,
    pub tile_count_y: usize,
    pub layers: Vec<Vec<usize>>,
    pub sprites: Vec<Sprite>,
}

impl Tilemap {
    pub fn new(
        tile_width: f32,
        tile_height: f32,
        tile_count_x: usize,
        tile_count_y: usize,
        layers: Vec<Vec<usize>>,
        sprites: Vec<Sprite>,
    ) -> Self {
        Self {
            tile_width,
            tile_height,
            tile_count_x,
            tile_count_y,
            layers,
            sprites,
        }
    }
}

impl Display for Tilemap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tilemap({}x{}, sprites={})",
            self.tile_count_x,
            self.tile_count_y,
            self.sprites.len()
        )
    }
}
