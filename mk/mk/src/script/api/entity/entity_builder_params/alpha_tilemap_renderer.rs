use super::EntityBuilderParam;
use crate::gfx::{AlphaTilemap, Color, Layer, ShaderHandle};
use anyhow::Context;
use fontdue::Font;
use mlua::prelude::*;
use std::sync::Arc;

pub struct AlphaTilemapRendererParams {
    pub layer: Layer,
    pub order: i32,
    pub color: Color,
    pub fore_shader: Arc<ShaderHandle>,
    pub back_shader: Arc<ShaderHandle>,
    pub font: Arc<Font>,
    pub font_size: f32,
    pub thickness: f32,
    pub smoothness: f32,
    pub tilemap: AlphaTilemap,
}

impl EntityBuilderParam for AlphaTilemapRendererParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            layer: table
                .get("layer")
                .with_context(|| "invalid value for 'layer' of AlphaTilemapRendererParams")
                .to_lua_err()?,
            order: table
                .get("order")
                .with_context(|| "invalid value for 'order' of AlphaTilemapRendererParams")
                .to_lua_err()?,
            color: table
                .get("color")
                .with_context(|| "invalid value for 'color' of AlphaTilemapRendererParams")
                .to_lua_err()?,
            fore_shader: table
                .get::<_, crate::script::api::gfx::Shader>("fore_shader")
                .with_context(|| "invalid value for 'fore_shader' of AlphaTilemapRendererParams")
                .to_lua_err()?
                .into_inner(),
            back_shader: table
                .get::<_, crate::script::api::gfx::Shader>("back_shader")
                .with_context(|| "invalid value for 'back_shader' of AlphaTilemapRendererParams")
                .to_lua_err()?
                .into_inner(),
            font: table
                .get::<_, crate::script::api::gfx::Font>("font")
                .with_context(|| "invalid value for 'font' of AlphaTilemapRendererParams")
                .to_lua_err()?
                .into_inner(),
            font_size: table
                .get("font_size")
                .with_context(|| "invalid value for 'font_size' of AlphaTilemapRendererParams")
                .to_lua_err()?,
            thickness: table
                .get("thickness")
                .with_context(|| "invalid value for 'thickness' of AlphaTilemapRendererParams")
                .to_lua_err()?,
            smoothness: table
                .get("smoothness")
                .with_context(|| "invalid value for 'smoothness' of AlphaTilemapRendererParams")
                .to_lua_err()?,
            tilemap: table
                .get("tilemap")
                .with_context(|| "invalid value for 'tilemap' of AlphaTilemapRendererParams")
                .to_lua_err()?,
        })
    }
}
