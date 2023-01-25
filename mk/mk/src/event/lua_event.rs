use super::{Event, ParamsToLuaTable};
use crate::script::FFITable;
use mlua::prelude::*;
use smartstring::alias::String;

pub struct LuaEvent {
    pub name: String,
    pub params: FFITable,
}

impl LuaEvent {
    pub fn new(name: String, params: FFITable) -> LuaEvent {
        LuaEvent { name, params }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn params(&self) -> &FFITable {
        &self.params
    }
}

impl<'lua> FromLuaMulti<'lua> for LuaEvent {
    fn from_lua_multi(mut values: LuaMultiValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let name: LuaString = if let Some(name) = values.pop_front() {
            FromLua::from_lua(name, lua)?
        } else {
            return Err(LuaError::external("Event name is missing"));
        };
        let params = if let Some(params) = values.pop_front() {
            FromLua::from_lua(params, lua)?
        } else {
            return Err(LuaError::external("Event params are missing"));
        };
        Ok(Self::new(String::from(name.to_str()?), params))
    }
}

impl Event for LuaEvent {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn param(&self, param_name: &str) -> Option<&dyn std::any::Any> {
        self.params.get(param_name).map(|p| p.as_any())
    }
}

impl ParamsToLuaTable for LuaEvent {
    fn params_to_lua_table<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        self.params.as_lua_table(lua)
    }
}
