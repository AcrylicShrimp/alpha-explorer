use mlua::prelude::*;

pub struct FFIError {
    error: LuaError,
}

impl FFIError {
    pub fn new(error: LuaError) -> Self {
        Self { error }
    }

    pub fn as_error(&self) -> &LuaError {
        &self.error
    }
}

impl<'lua> ToLua<'lua> for FFIError {
    fn to_lua(self, _lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::Error(self.error))
    }
}
