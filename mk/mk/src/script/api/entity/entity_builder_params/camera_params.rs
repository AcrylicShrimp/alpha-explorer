use super::EntityBuilderParam;
use crate::render::Layer;
use anyhow::Context;
use mlua::prelude::*;

pub struct CameraParams {
    pub layer: Layer,
    pub order: isize,
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
        })
    }
}
