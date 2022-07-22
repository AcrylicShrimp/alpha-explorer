use crate::render::{TexelMapping, Texture};
use fontdue::Font;
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
            texture: Texture::with_size_r_u8_smooth(2048, 2048).into(),
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

    pub fn glyph(
        &mut self,
        sdf: &[u8],
        sdf_width: usize,
        sdf_height: usize,
    ) -> Option<TexelMapping> {
        if 2048 < self.offset_y + sdf_height {
            return None;
        }

        if 2048 < self.offset_x + sdf_width {
            self.offset_x = 0;
            self.offset_y += self.line_height;
            self.line_height = sdf_height;

            if 2048 < self.offset_y + sdf_height {
                return None;
            }
        }

        let mapping = TexelMapping::new(
            (self.offset_x as _, self.offset_y as _),
            (
                (self.offset_x + sdf_width) as _,
                (self.offset_y + sdf_height) as _,
            ),
        );
        self.texture.update_texel(
            self.offset_x as _,
            self.offset_y as _,
            sdf_width as _,
            sdf_height as _,
            &sdf,
        );
        self.offset_x += sdf_width;
        self.line_height = max(self.line_height, sdf_height);

        Some(mapping)
    }
}
