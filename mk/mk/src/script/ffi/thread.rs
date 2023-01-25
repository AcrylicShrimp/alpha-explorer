use mlua::prelude::*;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FFIThread(Arc<LuaRegistryKey>);

impl FFIThread {
    pub fn new<'lua>(lua: &'lua Lua, t: LuaThread<'lua>) -> LuaResult<Self> {
        Ok(FFIThread(Arc::new(lua.create_registry_value(t)?)))
    }

    pub fn as_thread<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaThread<'lua>> {
        Ok(lua.registry_value(&self.0)?)
    }
}

impl<'lua> ToLua<'lua> for FFIThread {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::Thread(self.as_thread(lua)?))
    }
}
