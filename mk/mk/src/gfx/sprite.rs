use wgpu::{Sampler, TextureView};

use super::{SpriteSlice, SpriteTexelMapping};
use crate::handles::*;
use std::fmt::Display;

pub struct Sprite {
    texture: TextureHandle,
    view: TextureView,
    sampler: Sampler,
    mapping: SpriteTexelMapping,
    slice: Option<SpriteSlice>,
}

impl Sprite {
    pub fn new(
        texture: TextureHandle,
        view: TextureView,
        sampler: Sampler,
        mapping: SpriteTexelMapping,
        slice: Option<SpriteSlice>,
    ) -> Self {
        Self {
            texture,
            view,
            sampler,
            mapping,
            slice,
        }
    }

    pub fn texture(&self) -> &TextureHandle {
        &self.texture
    }

    pub fn view(&self) -> &TextureView {
        &self.view
    }

    pub fn sampler(&self) -> &Sampler {
        &self.sampler
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
