use super::EntityBuilderParam;
use crate::render::{Color, Layer, Shader, Tilemap};
use anyhow::Context;
use mlua::prelude::*;
use std::sync::Arc;

pub struct TilemapRendererParams {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub tilemap: Arc<Tilemap>,
}

impl EntityBuilderParam for TilemapRendererParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            layer: table
                .get("layer")
                .with_context(|| "invalid value for 'layer' of TilemapRendererParams")
                .to_lua_err()?,
            order: table
                .get("order")
                .with_context(|| "invalid value for 'order' of TilemapRendererParams")
                .to_lua_err()?,
            color: table
                .get("color")
                .with_context(|| "invalid value for 'color' of TilemapRendererParams")
                .to_lua_err()?,
            shader: table
                .get::<_, crate::script::api::render::Shader>("shader")
                .with_context(|| "invalid value for 'shader' of TilemapRendererParams")
                .to_lua_err()?
                .into_inner(),
            tilemap: table
                .get::<_, crate::script::api::render::Tilemap>("tilemap")
                .with_context(|| "invalid value for 'tilemap' of TilemapRendererParams")
                .to_lua_err()?
                .into_inner(),
        })
    }
}
