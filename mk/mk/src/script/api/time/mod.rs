use crate::script::api::LuaApiTable;
use mlua::prelude::*;

mod time;

pub use time::*;

pub struct TimeModule;

impl LuaApiTable for TimeModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Time", time::Time::create_api_table(lua)?)?;

        Ok(table)
    }
}
