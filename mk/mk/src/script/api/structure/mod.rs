use crate::script::api::LuaApiTable;
use mlua::prelude::*;

mod size;
mod vec2;

pub use size::*;
pub use vec2::*;

pub struct StructureModule;

impl LuaApiTable for StructureModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Size", size::Size::create_api_table(lua)?)?;
        table.set("Vec2", vec2::Vec2::create_api_table(lua)?)?;

        Ok(table)
    }
}
