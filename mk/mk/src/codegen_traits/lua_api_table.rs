use mlua::prelude::*;

pub trait LuaApiTable {
    fn api_name() -> &'static str;
    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()>;
}
