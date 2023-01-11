use crate::{
    gfx::{AlphaTilemap, Color, Layer},
    handles::*,
};
use specs::{prelude::*, Component};

#[derive(Component)]
pub struct AlphaTilemapRenderer {
    pub layer: Layer,
    pub order: i32,
    pub color: Color,
    pub fore_shader: ShaderHandle,
    pub back_shader: ShaderHandle,
    pub font: FontHandle,
    pub font_size: f32,
    pub thickness: f32,
    pub smoothness: f32,
    pub tilemap: AlphaTilemap,
}

impl AlphaTilemapRenderer {
    pub fn new(
        layer: Layer,
        order: i32,
        color: Color,
        fore_shader: ShaderHandle,
        back_shader: ShaderHandle,
        font: FontHandle,
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
