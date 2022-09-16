use crate::script::api::LuaApiTable;
use mlua::prelude::*;

mod audio_clip;

pub use audio_clip::*;

pub struct AudioModule;

impl LuaApiTable for AudioModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        Ok(table)
    }
}
