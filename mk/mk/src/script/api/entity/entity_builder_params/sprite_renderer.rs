use super::EntityBuilderParam;
use crate::render::{Color, Layer, Shader, Sprite};
use anyhow::Context;
use mlua::prelude::*;
use std::sync::Arc;

pub struct SpriteRendererParams {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub sprite: Arc<Sprite>,
}

impl EntityBuilderParam for SpriteRendererParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            layer: table
                .get("layer")
                .with_context(|| "invalid value for 'layer' of SpriteRendererParams")
                .to_lua_err()?,
            order: table
                .get("order")
                .with_context(|| "invalid value for 'order' of SpriteRendererParams")
                .to_lua_err()?,
            color: table
                .get("color")
                .with_context(|| "invalid value for 'color' of SpriteRendererParams")
                .to_lua_err()?,
            shader: table
                .get::<_, crate::script::api::render::Shader>("shader")
                .with_context(|| "invalid value for 'shader' of SpriteRendererParams")
                .to_lua_err()?
                .into_inner(),
            sprite: table
                .get::<_, crate::script::api::render::Sprite>("sprite")
                .with_context(|| "invalid value for 'sprite' of SpriteRendererParams")
                .to_lua_err()?
                .into_inner(),
        })
    }
}
