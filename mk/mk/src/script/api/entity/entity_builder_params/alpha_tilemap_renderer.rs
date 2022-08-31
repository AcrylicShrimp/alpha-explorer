use super::EntityBuilderParam;
use crate::render::{AlphaTilemap, Color, Layer, Shader};
use fontdue::Font;
use rhai::{EvalAltResult, INT};
use std::sync::Arc;

pub struct AlphaTilemapRendererParams {
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

impl EntityBuilderParam for AlphaTilemapRendererParams {
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
            fore_shader: table
                .remove("fore_shader")
                .ok_or_else(|| "the field 'fore_shader' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'fore_shader' is not valid type")?,
            back_shader: table
                .remove("back_shader")
                .ok_or_else(|| "the field 'back_shader' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'back_shader' is not valid type")?,
            font: table
                .remove("font")
                .ok_or_else(|| "the field 'font' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'font' is not valid type")?,
            font_size: table
                .remove("font_size")
                .ok_or_else(|| "the field 'font_size' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'font_size' is not valid type")?,
            thickness: table
                .remove("thickness")
                .ok_or_else(|| "the field 'thickness' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'thickness' is not valid type")?,
            smoothness: table
                .remove("smoothness")
                .ok_or_else(|| "the field 'smoothness' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'smoothness' is not valid type")?,
            tilemap: table
                .remove("tilemap")
                .ok_or_else(|| "the field 'tilemap' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'tilemap' is not valid type")?,
        })
    }
}
