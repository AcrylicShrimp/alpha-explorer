use super::EntityBuilderParam;
use crate::render::{Color, Layer, Shader, SpriteNinePatch};
use anyhow::Context;
use mlua::prelude::*;
use std::sync::Arc;

pub struct NinePatchRendererParams {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub nine_patch: Arc<SpriteNinePatch>,
}

impl EntityBuilderParam for NinePatchRendererParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            layer: table
                .get("layer")
                .with_context(|| "invalid value for 'layer' of NinePatchRendererParams")
                .to_lua_err()?,
            order: table
                .get("order")
                .with_context(|| "invalid value for 'order' of NinePatchRendererParams")
                .to_lua_err()?,
            color: table
                .get("color")
                .with_context(|| "invalid value for 'color' of NinePatchRendererParams")
                .to_lua_err()?,
            shader: table
                .get::<_, crate::script::api::render::Shader>("shader")
                .with_context(|| "invalid value for 'shader' of NinePatchRendererParams")
                .to_lua_err()?
                .into_inner(),
            nine_patch: table
                .get::<_, crate::script::api::render::SpriteNinePatch>("nine_patch")
                .with_context(|| "invalid value for 'nine_patch' of NinePatchRendererParams")
                .to_lua_err()?
                .into_inner(),
        })
    }
}
