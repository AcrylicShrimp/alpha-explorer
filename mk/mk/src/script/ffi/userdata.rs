use mlua::prelude::*;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FFIUserData(Arc<LuaRegistryKey>);

impl FFIUserData {
    pub fn new<'lua>(lua: &'lua Lua, u: LuaAnyUserData<'lua>) -> LuaResult<Self> {
        Ok(FFIUserData(Arc::new(lua.create_registry_value(u)?)))
    }

    pub fn as_userdata<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaAnyUserData<'lua>> {
        Ok(lua.registry_value(&self.0)?)
    }
}

impl<'lua> ToLua<'lua> for FFIUserData {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::UserData(self.as_userdata(lua)?))
    }
}
