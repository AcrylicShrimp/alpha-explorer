use super::EntityBuilderParam;
use crate::render::{Color, Layer, Shader, Tilemap};
use rhai::{EvalAltResult, INT};
use std::sync::Arc;

pub struct TilemapRendererParams {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub tilemap: Arc<Tilemap>,
}

impl EntityBuilderParam for TilemapRendererParams {
    fn from_table(mut table: rhai::Map) -> Result<Self, Box<EvalAltResult>> {
        Ok(Self {
            layer: table
                .remove("layer")
                .ok_or_else(|| "the field 'layer' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'layer' is not valid type")?,
            order: table
                .remove("order")
                .ok_or_else(|| "the field 'order' is not specified")?
                .try_cast::<INT>()
                .ok_or_else(|| "the field 'order' is not valid type")? as _,
            color: table
                .remove("color")
                .ok_or_else(|| "the field 'color' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'color' is not valid type")?,
            shader: table
                .remove("shader")
                .ok_or_else(|| "the field 'shader' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'shader' is not valid type")?,
            tilemap: table
                .remove("tilemap")
                .ok_or_else(|| "the field 'tilemap' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'tilemap' is not valid type")?,
        })
    }
}
