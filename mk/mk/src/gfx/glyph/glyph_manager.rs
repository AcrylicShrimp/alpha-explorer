use super::{generate_sdf, GlyphSprite, GlyphTexture};
use crate::{gfx::RenderManager, handles::*};
use fontdue::{layout::GlyphRasterConfig, Font};
use std::collections::HashMap;

pub struct GlyphManager {
    sdf_font_size: f32,
    sdf_inset: usize,
    sdf_radius: usize,
    sdf_cutoff: f32,
    glyphs: HashMap<GlyphRasterConfig, GlyphSprite>,
    glyph_textures: HashMap<*const Font, Vec<GlyphTexture>>,
}

impl GlyphManager {
    pub fn new(sdf_font_size: f32, sdf_inset: usize, sdf_radius: usize, sdf_cutoff: f32) -> Self {
        Self {
            sdf_font_size,
            sdf_inset,
            sdf_radius,
            sdf_cutoff,
            glyphs: HashMap::new(),
            glyph_textures: HashMap::new(),
        }
    }

    pub fn sdf_font_size(&self) -> f32 {
        self.sdf_font_size
    }

    pub fn sdf_inset(&self) -> usize {
        self.sdf_inset
    }

    pub fn glyph(
        &mut self,
        render_mgr: &RenderManager,
        font: &FontHandle,
        glyph: GlyphRasterConfig,
    ) -> &GlyphSprite {
        if !self.glyphs.contains_key(&glyph) {
            let (metrics, rasterized) = font
                .inner()
                .rasterize_indexed(glyph.glyph_index as _, self.sdf_font_size);
            let sdf = generate_sdf(
                &metrics,
                &rasterized,
                self.sdf_inset,
                self.sdf_radius,
                self.sdf_cutoff,
            );
            let glyph_textures = self
                .glyph_textures
                .entry(font.as_ptr())
                .or_insert_with(|| Vec::with_capacity(2));

            for glyph_texture in glyph_textures.iter_mut() {
                if let Some(mapping) = glyph_texture.glyph(
                    render_mgr,
                    (metrics.width + 2 * self.sdf_inset) as u16,
                    (metrics.height + 2 * self.sdf_inset) as u16,
                    &sdf,
                ) {
                    self.glyphs.insert(
                        glyph,
                        GlyphSprite::new(glyph_texture.texture().clone(), mapping),
                    );
                    return self.glyphs.get(&glyph).unwrap();
                }
            }

            let mut glyph_texture = GlyphTexture::new(render_mgr, font.clone());
            let mapping = glyph_texture
                .glyph(
                    render_mgr,
                    (metrics.width + 2 * self.sdf_inset) as u16,
                    (metrics.height + 2 * self.sdf_inset) as u16,
                    &sdf,
                )
                .unwrap();
            self.glyphs.insert(
                glyph,
                GlyphSprite::new(glyph_texture.texture().clone(), mapping),
            );
            glyph_textures.push(glyph_texture);
        }

        self.glyphs.get(&glyph).unwrap()
    }
}
