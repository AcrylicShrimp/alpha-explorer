use super::EntityBuilderParam;
use crate::render::{Color, Layer, Shader, SpriteNinePatch};
use rhai::{EvalAltResult, INT};
use std::sync::Arc;

pub struct NinePatchRendererParams {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub nine_patch: Arc<SpriteNinePatch>,
}

impl EntityBuilderParam for NinePatchRendererParams {
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
            nine_patch: table
                .remove("nine_patch")
                .ok_or_else(|| "the field 'nine_patch' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'nine_patch' is not valid type")?,
        })
    }
}
