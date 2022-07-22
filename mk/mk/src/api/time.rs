use crate::api::use_context;
use crate::codegen_traits::LuaApiTable;
use mlua::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Time;

impl LuaApiTable for Time {
    fn api_name() -> &'static str {
        "Time"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "dt",
            lua.create_function(|lua, _: ()| use_context().time_mgr().dt().to_lua(lua))?,
        )?;
        Ok(())
    }
}
