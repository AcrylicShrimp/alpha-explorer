use crate::{
    gfx::{Color, Layer, Tilemap},
    handles::*,
};
use specs::{prelude::*, Component};
use std::sync::Arc;

#[derive(Component)]
pub struct TilemapRenderer {
    pub layer: Layer,
    pub order: i32,
    pub color: Color,
    pub shader: ShaderHandle,
    pub tilemap: Arc<Tilemap>,
}

impl TilemapRenderer {
    pub fn new(
        layer: Layer,
        order: i32,
        color: Color,
        shader: ShaderHandle,
        tilemap: Arc<Tilemap>,
    ) -> Self {
        Self {
            layer,
            order,
            color,
            shader,
            tilemap,
        }
    }
}
