use super::EntityBuilderParam;
use crate::{
    gfx::{Color, GlyphLayoutConfig, Layer},
    handles::*,
};
use anyhow::Context;
use mlua::prelude::*;

pub struct GlyphRendererParams {
    pub layer: Layer,
    pub order: i32,
    pub color: Color,
    pub shader: ShaderHandle,
    pub thickness: f32,
    pub smoothness: f32,
    pub font: FontHandle,
    pub font_size: f32,
    pub text: Option<String>,
    pub config: Option<GlyphLayoutConfig>,
}

impl EntityBuilderParam for GlyphRendererParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            layer: table
                .get("layer")
                .with_context(|| "invalid value for 'layer' of GlyphRendererParams")
                .to_lua_err()?,
            order: table
                .get("order")
                .with_context(|| "invalid value for 'order' of GlyphRendererParams")
                .to_lua_err()?,
            color: table
                .get("color")
                .with_context(|| "invalid value for 'color' of GlyphRendererParams")
                .to_lua_err()?,
            shader: table
                .get("shader")
                .with_context(|| "invalid value for 'shader' of GlyphRendererParams")
                .to_lua_err()?,
            thickness: table
                .get("thickness")
                .with_context(|| "invalid value for 'thickness' of GlyphRendererParams")
                .to_lua_err()?,
            smoothness: table
                .get("smoothness")
                .with_context(|| "invalid value for 'smoothness' of GlyphRendererParams")
                .to_lua_err()?,
            font: table
                .get("font")
                .with_context(|| "invalid value for 'font' of GlyphRendererParams")
                .to_lua_err()?,
            font_size: table
                .get("font_size")
                .with_context(|| "invalid value for 'font_size' of GlyphRendererParams")
                .to_lua_err()?,
            text: table
                .get("text")
                .with_context(|| "invalid value for 'text' of GlyphRendererParams")
                .to_lua_err()?,
            config: table
                .get("config")
                .with_context(|| "invalid value for 'config' of GlyphRendererParams")
                .to_lua_err()?,
        })
    }
}
