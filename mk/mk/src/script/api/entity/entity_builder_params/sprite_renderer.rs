use super::EntityBuilderParam;
use crate::{
    gfx::{Color, Layer},
    handles::*,
};
use anyhow::Context;
use mlua::prelude::*;

pub struct SpriteRendererParams {
    pub layer: Layer,
    pub order: i32,
    pub color: Color,
    pub shader: ShaderHandle,
    pub sprite: SpriteHandle,
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
                .get("shader")
                .with_context(|| "invalid value for 'shader' of SpriteRendererParams")
                .to_lua_err()?,
            sprite: table
                .get("sprite")
                .with_context(|| "invalid value for 'sprite' of SpriteRendererParams")
                .to_lua_err()?,
        })
    }
}
