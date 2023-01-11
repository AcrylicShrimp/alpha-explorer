use super::EntityBuilderParam;
use anyhow::Context;
use mlua::prelude::*;

pub struct UIMaskParams {
    pub render_itself: bool,
}

impl EntityBuilderParam for UIMaskParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            render_itself: table
                .get("render_itself")
                .with_context(|| "invalid value for 'render_itself' of UIMaskParams")
                .to_lua_err()?,
        })
    }
}
