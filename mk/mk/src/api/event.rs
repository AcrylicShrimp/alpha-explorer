use crate::codegen_traits::LuaApiTable;
use crate::event::events::{
    Diagnostic, KeyDown, KeyUp, PointerDown, PointerEnter, PointerExit, PointerMove, PointerUp,
    PostRender, PostUpdate, PreRender, PreUpdate, Update,
};
use mlua::prelude::*;

use super::register_api_table;

#[derive(Debug, Clone, Copy)]
pub struct Event;

impl LuaApiTable for Event {
    fn api_name() -> &'static str {
        "Event"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        register_api_table::<Diagnostic>(lua, &table)?;
        register_api_table::<PreUpdate>(lua, &table)?;
        register_api_table::<Update>(lua, &table)?;
        register_api_table::<PostUpdate>(lua, &table)?;
        register_api_table::<PreRender>(lua, &table)?;
        register_api_table::<PostRender>(lua, &table)?;
        register_api_table::<KeyDown>(lua, &table)?;
        register_api_table::<KeyUp>(lua, &table)?;
        register_api_table::<PointerEnter>(lua, &table)?;
        register_api_table::<PointerExit>(lua, &table)?;
        register_api_table::<PointerMove>(lua, &table)?;
        register_api_table::<PointerDown>(lua, &table)?;
        register_api_table::<PointerUp>(lua, &table)?;
        Ok(())
    }
}
