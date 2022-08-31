use crate::render::AlphaTile;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct AlphaTileset {
    pub tiles: Vec<AlphaTile>,
}

impl AlphaTileset {
    pub fn new(tiles: Vec<AlphaTile>) -> Self {
        Self { tiles }
    }
}

impl Display for AlphaTileset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlphaTileset(tiles={})", self.tiles.len())
    }
}
