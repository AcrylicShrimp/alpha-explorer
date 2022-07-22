macro_rules! impl_event_type_lua_api {
    ($name:ident) => {
        impl crate::codegen_traits::LuaApiTable for $name {
            fn api_name() -> &'static str {
                stringify!($name)
            }

            fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
                table.set(
                    "event_type",
                    crate::event::EventType::from(std::any::TypeId::of::<Self>()),
                )?;
                table.set(
                    "listen",
                    lua.create_function(|lua, handler: mlua::Function| {
                        Ok(crate::api::use_context()
                            .event_mgr()
                            .dispatcher()
                            .add_listener::<Self>(crate::event::TypedEventListener::LuaFunction(
                                crate::util::BoxId::new(lua.create_registry_value(handler)?),
                            )))
                    })?,
                )?;
                table.set(
                    "unlisten",
                    lua.create_function(|_lua, handler: usize| {
                        Ok(crate::api::use_context()
                            .event_mgr()
                            .dispatcher()
                            .remove_listener::<Self>(handler))
                    })?,
                )?;
                Ok(())
            }
        }
    };
}

mod diagnostic;
mod input;
mod lifecycles;
mod per_entity;

pub use diagnostic::*;
pub use input::*;
pub use lifecycles::*;
pub use per_entity::*;
