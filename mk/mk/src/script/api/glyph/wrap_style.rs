use crate::script::api::LuaApiTable;
use mlua::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct WrapStyle(pub fontdue::layout::WrapStyle);

impl LuaApiTable for WrapStyle {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Word", WrapStyle(fontdue::layout::WrapStyle::Word))?;
        table.set("Letter", WrapStyle(fontdue::layout::WrapStyle::Letter))?;

        Ok(table)
    }
}

impl LuaUserData for WrapStyle {}
