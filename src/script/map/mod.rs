mod map;
mod map_size;
mod map_tile;

use map::*;
use map_size::*;
use map_tile::*;
use mk::{mlua::prelude::*, script::LuaApiTable};

#[derive(Debug, Clone, Copy)]
pub struct MapSubModule;

impl LuaApiTable for MapSubModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;
        Ok(table)
    }
}

// fn generate_map(width: u32, height: u32) -> Map {
//     todo!()
// }
