use crate::{engine::use_context, script::api::LuaApiTable};
use mlua::prelude::*;

pub struct Time;

impl LuaApiTable for Time {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "time",
            lua.create_function(|_lua, ()| {
                let time_mgr = use_context().time_mgr();
                Ok(time_mgr.time_f64())
            })?,
        )?;
        table.set(
            "dt",
            lua.create_function(|_lua, ()| {
                let time_mgr = use_context().time_mgr();
                Ok(time_mgr.dt_f64())
            })?,
        )?;

        Ok(table)
    }
}
