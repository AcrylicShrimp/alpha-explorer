use crate::{
    gfx::{RenderManager, SpriteTexelMapping},
    handles::*,
};
use std::cmp::max;

pub struct GlyphTexture {
    texture: TextureHandle,
    font: FontHandle,
    offset_x: u16,
    offset_y: u16,
    line_height: u16,
}

impl GlyphTexture {
    pub fn new(render_mgr: &RenderManager, font: FontHandle) -> Self {
        Self {
            texture: render_mgr.create_glyph_texture(2048u16, 2048u16),
            font,
            offset_x: 0,
            offset_y: 0,
            line_height: 0,
        }
    }

    pub fn font(&self) -> &FontHandle {
        &self.font
    }

    pub fn texture(&self) -> &TextureHandle {
        &self.texture
    }

    pub fn glyph(
        &mut self,
        render_mgr: &RenderManager,
        sdf_width: u16,
        sdf_height: u16,
        sdf: &[u8],
    ) -> Option<SpriteTexelMapping> {
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

        let mapping = SpriteTexelMapping::new(
            self.offset_x as _,
            (self.offset_x + sdf_width) as _,
            self.offset_y as _,
            (self.offset_y + sdf_height) as _,
        );
        render_mgr.update_glyph_texture(
            &self.texture,
            self.offset_x as u32,
            self.offset_y as u32,
            sdf_width as u32,
            sdf_height as u32,
            &sdf,
        );
        self.offset_x += sdf_width;
        self.line_height = max(self.line_height, sdf_height);

        Some(mapping)
    }
}
