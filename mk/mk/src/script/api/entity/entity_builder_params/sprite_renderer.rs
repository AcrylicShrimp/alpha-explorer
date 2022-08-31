use super::EntityBuilderParam;
use crate::render::{Color, Layer, Shader, Sprite};
use rhai::{EvalAltResult, INT};
use std::sync::Arc;

pub struct SpriteRendererParams {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub sprite: Arc<Sprite>,
}

impl EntityBuilderParam for SpriteRendererParams {
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
            sprite: table
                .remove("sprite")
                .ok_or_else(|| "the field 'sprite' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'sprite' is not valid type")?,
        })
    }
}
