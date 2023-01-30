use crate::script::api::LuaApiTable;
use mlua::prelude::*;

macro_rules! impl_event_listeners {
    ($lua:ident, $table:ident) => {
        $table.set(
            "listen",
            $lua.create_function(|lua, handler: mlua::Function| {
                let event_mgr = crate::engine::use_context().event_mgr();
                Ok(event_mgr.add_handler(
                    <Self as crate::event::NativeEvent>::name(),
                    crate::event::EventHandler::lua(crate::script::FFIFunction::new(lua, handler)?),
                ))
            })?,
        )?;
        $table.set(
            "unlisten",
            $lua.create_function(|_lua, handler: crate::event::EventHandler| {
                let event_mgr = crate::engine::use_context().event_mgr();
                event_mgr.remove_handler(<Self as crate::event::NativeEvent>::name(), handler);
                Ok(())
            })?,
        )?;
    };
}

mod diagnostic;
mod input;
mod lifecycles;
mod ui;

pub use diagnostic::*;
pub use input::*;
pub use lifecycles::*;
pub use ui::*;

pub struct EventModule;

impl LuaApiTable for EventModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "DiagnosticLevel",
            diagnostic::DiagnosticLevel::create_api_table(lua)?,
        )?;
        table.set("KeyDown", input::KeyDown::create_api_table(lua)?)?;
        table.set("KeyUp", input::KeyUp::create_api_table(lua)?)?;
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
