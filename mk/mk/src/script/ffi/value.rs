use mlua::prelude::*;
use std::any::Any;

pub trait FFIValue {
    fn clone(&self) -> Box<dyn FFIValue>;
    fn as_any(&self) -> &dyn Any;
    fn as_lua_value<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>>;
    fn to_lua_value<'lua>(self: Box<Self>, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>>;
}

impl<T> FFIValue for T
where
    T: Any + Clone + for<'lua> ToLua<'lua>,
{
    fn clone(&self) -> Box<dyn FFIValue> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_lua_value<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.pack(self.clone())
    }

    fn to_lua_value<'lua>(self: Box<Self>, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.pack(*self)
    }
}
