mod map;

use map::*;
use mk::{mlua::prelude::*, script::LuaApiTable};

pub struct GameModule;

impl LuaApiTable for GameModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;
        table.set("map", MapSubModule::create_api_table(lua)?)?;
        Ok(table)
    }
}
