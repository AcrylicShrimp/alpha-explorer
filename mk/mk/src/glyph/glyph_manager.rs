use crate::glyph::{generate_sdf, GlyphTexture};
use crate::render::{TexelMapping, Texture};
use fontdue::layout::GlyphRasterConfig;
use fontdue::Font;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Glyph {
    pub texture: Arc<Texture>,
    pub mapping: TexelMapping,
}

#[derive(Debug)]
pub struct GlyphManager {
    sdf_font_size: f32,
    sdf_inset: usize,
    sdf_radius: usize,
    sdf_cutoff: f32,
    glyphs: HashMap<GlyphRasterConfig, Glyph>,
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

    pub fn glyph(&mut self, font: &Arc<Font>, glyph: GlyphRasterConfig) -> &Glyph {
        if !self.glyphs.contains_key(&glyph) {
            let (metrics, rasterized) =
                font.rasterize_indexed(glyph.glyph_index as _, self.sdf_font_size);
            let sdf = generate_sdf(
                &metrics,
                &rasterized,
                self.sdf_inset,
                self.sdf_radius,
                self.sdf_cutoff,
            );
            let glyph_textures = self
                .glyph_textures
                .entry(Arc::as_ptr(font))
                .or_insert_with(|| Vec::with_capacity(2));

            for glyph_texture in glyph_textures.iter_mut() {
                if let Some(mapping) = glyph_texture.glyph(
                    &sdf,
                    metrics.width + 2 * self.sdf_inset,
                    metrics.height + 2 * self.sdf_inset,
                ) {
                    self.glyphs.insert(
                        glyph,
                        Glyph {
                            texture: glyph_texture.texture().clone(),
                            mapping,
                        },
                    );
                    return self.glyphs.get(&glyph).unwrap();
                }
            }

            let mut glyph_texture = GlyphTexture::new(font.clone());
            let mapping = glyph_texture
                .glyph(
                    &sdf,
                    metrics.width + 2 * self.sdf_inset,
                    metrics.height + 2 * self.sdf_inset,
                )
                .unwrap();
            self.glyphs.insert(
                glyph,
                Glyph {
                    texture: glyph_texture.texture().clone(),
                    mapping,
                },
            );
            glyph_textures.push(glyph_texture);
        }

        self.glyphs.get(&glyph).unwrap()
    }
}
