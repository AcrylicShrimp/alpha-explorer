use crate::render::*;
use fontdue::layout::GlyphRasterConfig;
use fontdue::Font;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GlyphManager {
    glyphs: HashMap<GlyphRasterConfig, (Arc<Texture>, TexelMapping)>,
    glyph_textures: HashMap<*const Font, Vec<GlyphTexture>>,
}

impl GlyphManager {
    pub fn new() -> Self {
        Self {
            glyphs: HashMap::new(),
            glyph_textures: HashMap::new(),
        }
    }

    pub fn glyph<'g>(
        &'g mut self,
        font: &Arc<Font>,
        glyph: GlyphRasterConfig,
    ) -> &'g (Arc<Texture>, TexelMapping) {
        if 2048f32 < glyph.px {
            panic!("glyph is over-sized");
        }

        if !self.glyphs.contains_key(&glyph) {
            let (metrics, rasterized) = font.rasterize_indexed(glyph.glyph_index as _, glyph.px);
            let glyph_textures = self
                .glyph_textures
                .entry(Arc::as_ptr(font))
                .or_insert_with(|| Vec::with_capacity(2));

            for glyph_texture in glyph_textures.iter_mut() {
                if let Some(mapping) = glyph_texture.glyph(&metrics, &rasterized) {
                    self.glyphs
                        .insert(glyph, (glyph_texture.texture().clone(), mapping));
                    return self.glyphs.get(&glyph).unwrap();
                }
            }

            let mut glyph_texture = GlyphTexture::new(font.clone());
            let mapping = glyph_texture.glyph(&metrics, &rasterized).unwrap();
            self.glyphs
                .insert(glyph, (glyph_texture.texture().clone(), mapping));
            glyph_textures.push(glyph_texture);
        }

        self.glyphs.get(&glyph).unwrap()
    }
}
