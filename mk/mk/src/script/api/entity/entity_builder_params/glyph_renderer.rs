use super::EntityBuilderParam;
use crate::{
    component::GlyphLayoutConfig,
    render::{Color, Layer, Shader},
    script::api::extract_float,
};
use fontdue::Font;
use rhai::{EvalAltResult, ImmutableString, INT};
use std::sync::Arc;

pub struct GlyphRendererParams {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub thickness: f32,
    pub smoothness: f32,
    pub font: Arc<Font>,
    pub font_size: f32,
    pub text: Option<ImmutableString>,
    pub config: Option<GlyphLayoutConfig>,
}

impl EntityBuilderParam for GlyphRendererParams {
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
            thickness: extract_float(
                table
                    .remove("thickness")
                    .ok_or_else(|| "the field 'thickness' is not specified")?,
            )?,
            smoothness: extract_float(
                table
                    .remove("smoothness")
                    .ok_or_else(|| "the field 'smoothness' is not specified")?,
            )?,
            font: table
                .remove("font")
                .ok_or_else(|| "the field 'font' is not specified")?
                .try_cast()
                .ok_or_else(|| "the field 'font' is not valid type")?,
            font_size: extract_float(
                table
                    .remove("font_size")
                    .ok_or_else(|| "the field 'font_size' is not specified")?,
            )?,
            text: table
                .remove("text")
                .map(|text| {
                    text.try_cast()
                        .ok_or_else(|| "the field 'text' is not valid type")
                })
                .transpose()?,
            config: table
                .remove("config")
                .map(|config| {
                    config
                        .try_cast()
                        .ok_or_else(|| "the field 'config' is not valid type")
                })
                .transpose()?,
        })
    }
}
