use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type UIScaleMode = crate::ui::UIScaleMode;

impl LuaApiTable for UIScaleMode {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Constant", UIScaleMode::Constant)?;
        table.set("Stretch", UIScaleMode::Stretch)?;
        table.set("Fit", UIScaleMode::Fit)?;
        table.set("Fill", UIScaleMode::Fill)?;
        table.set("MatchWidth", UIScaleMode::MatchWidth)?;
        table.set("MatchHeight", UIScaleMode::MatchHeight)?;

        Ok(table)
    }
}

impl LuaUserData for UIScaleMode {}
