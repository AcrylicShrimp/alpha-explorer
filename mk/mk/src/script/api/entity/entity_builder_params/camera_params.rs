use super::EntityBuilderParam;
use crate::gfx::{ClearMode, Color, Layer};
use anyhow::Context;
use mlua::prelude::*;

pub struct CameraParams {
    pub layer: Layer,
    pub order: isize,
    pub clear_mode: ClearMode,
    pub clear_color: Color,
}

impl EntityBuilderParam for CameraParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            layer: table
                .get("layer")
                .with_context(|| "invalid value for 'layer' of CameraParams")
                .to_lua_err()?,
            order: table
                .get("order")
                .with_context(|| "invalid value for 'order' of CameraParams")
                .to_lua_err()?,
            clear_mode: table
                .get("clear_mode")
                .with_context(|| "invalid value for 'clear_mode' of CameraParams")
                .to_lua_err()?,
            clear_color: table
                .get("clear_color")
                .with_context(|| "invalid value for 'clear_color' of CameraParams")
                .to_lua_err()?,
        })
    }
}
