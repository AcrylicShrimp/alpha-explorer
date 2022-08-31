use crate::render::{AlphaTilemap, Color, Layer, Shader};
use fontdue::Font;
use std::sync::Arc;

pub struct AlphaTilemapRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub fore_shader: Arc<Shader>,
    pub back_shader: Arc<Shader>,
    pub font: Arc<Font>,
    pub font_size: f32,
    pub thickness: f32,
    pub smoothness: f32,
    pub tilemap: AlphaTilemap,
}

impl AlphaTilemapRenderer {
    pub fn new(
        layer: Layer,
        order: isize,
        color: Color,
        fore_shader: Arc<Shader>,
        back_shader: Arc<Shader>,
        font: Arc<Font>,
        font_size: f32,
        thickness: f32,
        smoothness: f32,
        tilemap: AlphaTilemap,
    ) -> Self {
        Self {
            layer,
            order,
            color,
            fore_shader,
            back_shader,
            font,
            font_size,
            thickness,
            smoothness,
            tilemap,
        }
    }
}
