use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type ClearMode = crate::gfx::ClearMode;

impl LuaApiTable for ClearMode {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("None", Self::None)?;
        table.set("Color", Self::Color)?;

        Ok(table)
    }
}

impl LuaUserData for ClearMode {}
