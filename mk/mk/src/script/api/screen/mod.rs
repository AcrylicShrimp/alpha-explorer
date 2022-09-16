use crate::script::api::LuaApiTable;
use mlua::prelude::*;

mod screen;

pub use screen::*;

pub struct ScreenModule;

impl LuaApiTable for ScreenModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Screen", screen::Screen::create_api_table(lua)?)?;

        Ok(table)
    }
}
