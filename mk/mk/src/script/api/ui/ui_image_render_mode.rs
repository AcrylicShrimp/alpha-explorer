use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type UIImageRenderMode = crate::ui::UIImageRenderMode;

impl LuaApiTable for UIImageRenderMode {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Simple", UIImageRenderMode::Simple)?;
        table.set("NinePatched", UIImageRenderMode::NinePatched)?;

        Ok(table)
    }
}

impl LuaUserData for UIImageRenderMode {}
