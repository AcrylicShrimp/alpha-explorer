use crate::script::api::LuaApiTable;
use mlua::prelude::*;

macro_rules! impl_event_listeners {
    ($lua:ident, $table:ident) => {
        $table.set(
            "listen",
            $lua.create_function(|lua, handler: mlua::Function| {
                let event_mgr = crate::engine::use_context().event_mgr();
                Ok(event_mgr.dispatcher().add_listener::<Self>(
                    crate::event::TypedEventListener::Script(crate::util::BoxId::new(
                        lua.create_registry_value(handler)?,
                    )),
                ))
            })?,
        )?;
        $table.set(
            "unlisten",
            $lua.create_function(|lua, handler: usize| {
                let event_mgr = crate::engine::use_context().event_mgr();
                if let Some(listener) = event_mgr.dispatcher().remove_listener::<Self>(handler) {
                    if let Ok(handler) = listener.into_inner().downcast::<mlua::RegistryKey>() {
                        lua.remove_registry_value(*handler)?;
                    }
                }
                Ok(())
            })?,
        )?;
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

pub struct EventModule;

impl LuaApiTable for EventModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "DiagnosticLevel",
            diagnostic::DiagnosticLevel::create_api_table(lua)?,
        )?;
        table.set("KeyDownEvent", input::KeyDownEvent::create_api_table(lua)?)?;
        table.set("KeyUpEvent", input::KeyUpEvent::create_api_table(lua)?)?;
        table.set("PointerEnter", input::PointerEnter::create_api_table(lua)?)?;
        table.set("PointerExit", input::PointerExit::create_api_table(lua)?)?;
        table.set("PointerMove", input::PointerMove::create_api_table(lua)?)?;
        table.set("PointerDown", input::PointerDown::create_api_table(lua)?)?;
        table.set("PointerUp", input::PointerUp::create_api_table(lua)?)?;
        table.set("PreUpdate", lifecycles::PreUpdate::create_api_table(lua)?)?;
        table.set("Update", lifecycles::Update::create_api_table(lua)?)?;
        table.set("PostUpdate", lifecycles::PostUpdate::create_api_table(lua)?)?;
        table.set("PreRender", lifecycles::PreRender::create_api_table(lua)?)?;
        table.set("PostRender", lifecycles::PostRender::create_api_table(lua)?)?;

        Ok(table)
    }
}
