use crate::api::*;
use mlua::prelude::*;

// pub fn lua_api_coroutine(lua: &Lua) -> LuaResult<LuaTable> {
//     let table = lua.create_table()?;

//     table.set(
//         "execute",
//         LuaValue::Function(lua.create_function(lua_api_coroutine_execute)?),
//     )?;
//     table.set(
//         "stop",
//         LuaValue::Function(lua.create_function(lua_api_coroutine_stop)?),
//     )?;

//     Ok(table)
// }

// fn lua_api_coroutine_execute(
//     _: &Lua,
//     (function, args): (LuaFunction, LuaMultiValue),
// ) -> LuaResult<Option<usize>> {
//     use_context().lua_mgr().execute_coroutine(function, args)
// }

// fn lua_api_coroutine_stop(_: &Lua, hash: usize) -> LuaResult<()> {
//     use_context().lua_mgr().stop_coroutine(hash);
//     Ok(())
// }
