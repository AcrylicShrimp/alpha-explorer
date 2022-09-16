use crate::{engine::use_context, script::api::LuaApiTable, structure::Size};
use mlua::prelude::*;

pub struct Screen;

impl LuaApiTable for Screen {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "size",
            lua.create_function(|_lua, ()| {
                let screen_mgr = use_context().screen_mgr();
                Ok(Size::new(
                    screen_mgr.width() as f32,
                    screen_mgr.height() as f32,
                ))
            })?,
        )?;
        table.set(
            "physical_size",
            lua.create_function(|_lua, ()| {
                let screen_mgr = use_context().screen_mgr();
                Ok(Size::new(
                    screen_mgr.physical_width() as f32,
                    screen_mgr.physical_height() as f32,
                ))
            })?,
        )?;
        table.set(
            "scale_factor",
            lua.create_function(|_lua, ()| {
                let screen_mgr = use_context().screen_mgr();
                Ok(screen_mgr.scale_factor())
            })?,
        )?;

        Ok(table)
    }
}
