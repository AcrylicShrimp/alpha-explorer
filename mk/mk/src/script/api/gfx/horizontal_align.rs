use crate::script::api::LuaApiTable;
use mlua::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct HorizontalAlign(pub fontdue::layout::HorizontalAlign);

impl LuaApiTable for HorizontalAlign {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Left", Self(fontdue::layout::HorizontalAlign::Left))?;
        table.set("Center", Self(fontdue::layout::HorizontalAlign::Center))?;
        table.set("Right", Self(fontdue::layout::HorizontalAlign::Right))?;

        Ok(table)
    }
}

impl LuaUserData for HorizontalAlign {}
