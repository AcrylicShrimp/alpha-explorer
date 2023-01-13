use super::{SpriteSlice, SpriteTexelMapping};
use crate::handles::*;
use std::fmt::Display;

pub struct Sprite {
    texture: TextureHandle,
    mapping: SpriteTexelMapping,
    slice: Option<SpriteSlice>,
}

impl Sprite {
    pub fn new(
        texture: TextureHandle,
        mapping: SpriteTexelMapping,
        slice: Option<SpriteSlice>,
    ) -> Self {
        Self {
            texture,
            mapping,
            slice,
        }
    }

    pub fn texture(&self) -> &TextureHandle {
        &self.texture
    }

    pub fn mapping(&self) -> SpriteTexelMapping {
        self.mapping
    }

    pub fn slice(&self) -> Option<SpriteSlice> {
        self.slice
    }

    pub fn width(&self) -> u32 {
        self.mapping.width() as u32
    }

    pub fn height(&self) -> u32 {
        self.mapping.height() as u32
    }
}

impl Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sprite({}x{})", self.width(), self.height())
    }
}
