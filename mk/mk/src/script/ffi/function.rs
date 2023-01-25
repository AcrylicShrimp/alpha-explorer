use mlua::prelude::*;
use std::{hash::Hash, sync::Arc};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FFIFunction(Arc<LuaRegistryKey>);

impl FFIFunction {
    pub fn new<'lua>(lua: &'lua Lua, f: LuaFunction<'lua>) -> LuaResult<Self> {
        Ok(FFIFunction(Arc::new(lua.create_registry_value(f)?)))
    }

    pub fn as_function<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaFunction<'lua>> {
        Ok(lua.registry_value(&self.0)?)
    }

    pub fn as_registry_key(&self) -> &LuaRegistryKey {
        &self.0
    }
}

impl<'lua> ToLua<'lua> for FFIFunction {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::Function(self.as_function(lua)?))
    }
}
