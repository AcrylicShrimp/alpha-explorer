mod asset;
mod coroutine;
mod entity;
mod event;
mod lua_manager;
mod screen;
mod time;

pub use asset::*;
pub use coroutine::*;
pub use entity::*;
pub use event::*;
pub use lua_manager::*;
pub use screen::*;
pub use time::*;

use crate::codegen_traits::LuaApiTable;
use crate::component::Transform;
use crate::render::Color;
use crate::structure::Vec2;
use mlua::prelude::*;

pub fn register_api_table<'lua, T>(
    lua: &'lua Lua,
    root_table: &'lua LuaTable<'lua>,
) -> LuaResult<()>
where
    T: LuaApiTable,
{
    let table = lua.create_table()?;
    <T as LuaApiTable>::fill_api_table(lua, &table)?;
    root_table.set(T::api_name(), table)?;
    Ok(())
}

pub fn lua_api(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    register_api_table::<Color>(lua, &table)?;
    register_api_table::<Entity>(lua, &table)?;
    register_api_table::<Event>(lua, &table)?;
    register_api_table::<FontAsset>(lua, &table)?;
    register_api_table::<Screen>(lua, &table)?;
    register_api_table::<ShaderAsset>(lua, &table)?;
    register_api_table::<SpriteAsset>(lua, &table)?;
    register_api_table::<SpriteAtlasAsset>(lua, &table)?;
    register_api_table::<SpriteAtlasGridAsset>(lua, &table)?;
    register_api_table::<SpriteNinePatchAsset>(lua, &table)?;
    register_api_table::<Time>(lua, &table)?;
    register_api_table::<Transform>(lua, &table)?;
    register_api_table::<Vec2>(lua, &table)?;
    // table.set("Coroutine", lua_api_coroutine(lua)?)?;

    Ok(table)
}
