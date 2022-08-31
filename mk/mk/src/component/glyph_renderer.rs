use crate::{
    render::{Color, Layer, Shader},
    structure::Size,
};
use fontdue::{
    layout::{
        CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign,
        WrapStyle,
    },
    Font,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct GlyphLayoutConfig {
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
    pub wrap_style: WrapStyle,
    pub wrap_hard_breaks: bool,
}

impl GlyphLayoutConfig {
    pub fn new(
        horizontal_align: HorizontalAlign,
        vertical_align: VerticalAlign,
        wrap_style: WrapStyle,
        wrap_hard_breaks: bool,
    ) -> Self {
        Self {
            horizontal_align,
            vertical_align,
            wrap_style,
            wrap_hard_breaks,
        }
    }
}

impl Default for GlyphLayoutConfig {
    fn default() -> Self {
        Self {
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Top,
            wrap_style: WrapStyle::Word,
            wrap_hard_breaks: true,
        }
    }
}

pub struct GlyphRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub thickness: f32,
    pub smoothness: f32,
    font: Arc<Font>,
    font_size: f32,
    text: String,
    config: GlyphLayoutConfig,
    layout: Layout,
}

impl GlyphRenderer {
    pub fn new(
        layer: Layer,
        order: isize,
        color: Color,
        shader: Arc<Shader>,
        thickness: f32,
        smoothness: f32,
        font: Arc<Font>,
        font_size: f32,
    ) -> Self {
        Self {
            layer,
            order,
            color,
            shader,
            thickness,
            smoothness,
            font,
            font_size,
            text: String::new(),
            config: GlyphLayoutConfig::default(),
            layout: Layout::new(CoordinateSystem::PositiveYUp),
        }
    }

    pub fn font(&self) -> &Arc<Font> {
        &self.font
    }

    pub fn set_font(&mut self, font: Arc<Font>) {
        self.font = font;
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn config(&self) -> &GlyphLayoutConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: GlyphLayoutConfig) {
        self.config = config;
        self.layout.reset(&LayoutSettings {
            x: 0f32,
            y: 0f32,
            max_width: None,
            max_height: None,
            horizontal_align: self.config.horizontal_align,
            vertical_align: self.config.vertical_align,
            wrap_style: self.config.wrap_style,
            wrap_hard_breaks: self.config.wrap_hard_breaks,
        });
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn font_and_layout(&mut self) -> (&Arc<Font>, &mut Layout) {
        (&self.font, &mut self.layout)
    }

    pub fn compute_size(&self) -> Size {
        let mut width = 0f32;

        for glyph in self.layout.glyphs() {
            width = width.max(glyph.x + glyph.width as f32);
        }

        Size::new(width, self.layout.height())
    }
}
