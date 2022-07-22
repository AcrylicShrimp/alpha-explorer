use crate::render::*;
use fontdue::{Font, Metrics};
use std::cmp::max;
use std::sync::Arc;

#[derive(Debug)]
pub struct GlyphTexture {
    texture: Arc<Texture>,
    font: Arc<Font>,
    offset_x: usize,
    offset_y: usize,
    line_height: usize,
}

impl GlyphTexture {
    pub fn new(font: Arc<Font>) -> Self {
        Self {
            texture: Texture::with_size_r_u8(2048, 2048).into(),
            font,
            offset_x: 0,
            offset_y: 0,
            line_height: 0,
        }
    }

    pub fn font(&self) -> &Arc<Font> {
        &self.font
    }

    pub fn texture(&self) -> &Arc<Texture> {
        &self.texture
    }

    pub fn glyph(&mut self, metrics: &Metrics, rasterized: &[u8]) -> Option<TexelMapping> {
        if 2048 < self.offset_y + metrics.height {
            return None;
        }

        if 2048 < self.offset_x + metrics.width {
            self.offset_x = 0;
            self.offset_y += self.line_height;
            self.line_height = metrics.height;

            if 2048 < self.offset_y + metrics.height {
                return None;
            }
        }

        let mapping = TexelMapping::new(
            (self.offset_x as _, self.offset_y as _),
            (
                (self.offset_x + metrics.width) as _,
                (self.offset_y + metrics.height) as _,
            ),
        );
        self.texture.update_texel(
            self.offset_x as _,
            self.offset_y as _,
            metrics.width as _,
            metrics.height as _,
            &rasterized,
        );
        self.offset_x += metrics.width;
        self.line_height = max(self.line_height, metrics.height);

        Some(mapping)
    }
}
