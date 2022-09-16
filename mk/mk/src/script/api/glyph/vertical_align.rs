use crate::script::api::LuaApiTable;
use mlua::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct VerticalAlign(pub fontdue::layout::VerticalAlign);

impl LuaApiTable for VerticalAlign {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Top", VerticalAlign(fontdue::layout::VerticalAlign::Top))?;
        table.set(
            "Middle",
            VerticalAlign(fontdue::layout::VerticalAlign::Middle),
        )?;
        table.set(
            "Bottom",
            VerticalAlign(fontdue::layout::VerticalAlign::Bottom),
        )?;

        Ok(table)
    }
}

impl LuaUserData for VerticalAlign {}
