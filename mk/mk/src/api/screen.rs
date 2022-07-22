use crate::api::use_context;
use crate::codegen_traits::LuaApiTable;
use crate::structure::Size;
use mlua::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Screen;

impl LuaApiTable for Screen {
    fn api_name() -> &'static str {
        "Screen"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "size",
            lua.create_function(|lua, _: ()| {
                let screen_mgr = use_context().screen_mgr();
                Size::new(screen_mgr.width() as f32, screen_mgr.height() as f32).to_lua(lua)
            })?,
        )?;
        table.set(
            "physical_size",
            lua.create_function(|lua, _: ()| {
                let screen_mgr = use_context().screen_mgr();
                Size::new(
                    screen_mgr.physical_width() as f32,
                    screen_mgr.physical_height() as f32,
                )
                .to_lua(lua)
            })?,
        )?;
        table.set(
            "scale_factor",
            lua.create_function(|lua, _: ()| {
                let screen_mgr = use_context().screen_mgr();
                screen_mgr.scale_factor().to_lua(lua)
            })?,
        )?;
        Ok(())
    }
}
