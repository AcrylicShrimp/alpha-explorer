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

#[macro_export]
macro_rules! arc_user_data {
    ($ty:ty => $name:ident) => {
        #[derive(Debug, Hash)]
        pub struct $name(*const $ty);

        impl $name {
            pub fn into_inner(self) -> std::sync::Arc<$ty> {
                std::sync::Arc::<$ty>::from(self)
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                unsafe {
                    std::sync::Arc::increment_strong_count(self.0);
                }

                Self(self.0)
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    std::sync::Arc::decrement_strong_count(self.0);
                }
            }
        }

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self(std::sync::Arc::into_raw(std::sync::Arc::new(value)))
            }
        }

        impl From<std::sync::Arc<$ty>> for $name {
            fn from(value: std::sync::Arc<$ty>) -> Self {
                Self(std::sync::Arc::into_raw(value))
            }
        }

        impl From<$name> for std::sync::Arc<$ty> {
            fn from(value: $name) -> Self {
                let arc = unsafe { std::sync::Arc::from_raw(value.0) };
                std::mem::forget(value);
                arc
            }
        }
    };
}

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
