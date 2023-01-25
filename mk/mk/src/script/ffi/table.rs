use super::{FFIFunction, FFIThread, FFIUserData, FFIValue};
use mlua::prelude::*;
use smartstring::alias::String;
use std::collections::HashMap;

#[derive(Default)]
pub struct FFITable {
    map: HashMap<String, Box<dyn FFIValue>>,
}

impl FFITable {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn from_lua_table<'lua>(lua: &'lua Lua, table: LuaTable<'lua>) -> LuaResult<Self> {
        lua_table_to_ffi_table(lua, table)
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&Box<dyn FFIValue>> {
        self.map.get(key.as_ref())
    }

    pub fn insert(&mut self, key: impl Into<String>, value: Box<dyn FFIValue>) {
        self.map.insert(key.into(), value);
    }

    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<Box<dyn FFIValue>> {
        self.map.remove(key.as_ref())
    }

    pub fn as_lua_table<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;
        for (key, value) in &self.map {
            table.set(lua.create_string(key.as_str())?, value.as_lua_value(lua)?)?;
        }
        Ok(table)
    }

    pub fn to_lua_table<'lua>(self, lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;
        for (key, value) in self.map {
            table.set(lua.create_string(key.as_str())?, value.to_lua_value(lua)?)?;
        }
        Ok(table)
    }
}

impl Clone for FFITable {
    fn clone(&self) -> Self {
        let mut map = HashMap::new();
        for (key, value) in &self.map {
            map.insert(key.clone(), value.as_ref().clone());
        }
        Self { map }
    }
}

impl<'lua> FromLua<'lua> for FFITable {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => Self::from_lua_table(lua, table),
            _ => Err(LuaError::external(format!(
                "Expected table, got {:?}",
                value
            ))),
        }
    }
}

impl<'lua> ToLua<'lua> for FFITable {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::Table(self.to_lua_table(lua)?))
    }
}

fn lua_table_to_ffi_table<'lua>(lua: &'lua Lua, table: LuaTable<'lua>) -> LuaResult<FFITable> {
    let mut ffi_table = FFITable::new();
    for pair in table.pairs::<LuaString, LuaValue>() {
        let (key, value) = pair?;
        let key = String::from(key.to_str()?);
        let value = match value {
            LuaValue::Nil => continue,
            LuaValue::Boolean(value) => Box::new(value) as Box<dyn FFIValue>,
            LuaValue::Integer(value) => Box::new(value) as Box<dyn FFIValue>,
            LuaValue::Number(value) => Box::new(value) as Box<dyn FFIValue>,
            LuaValue::String(value) => Box::new(value.to_str()?.to_string()) as Box<dyn FFIValue>,
            LuaValue::Function(value) => {
                Box::new(FFIFunction::new(lua, value)?) as Box<dyn FFIValue>
            }
            LuaValue::Table(value) => {
                Box::new(lua_table_to_ffi_table(lua, value)?) as Box<dyn FFIValue>
            }
            LuaValue::Thread(value) => Box::new(FFIThread::new(lua, value)?) as Box<dyn FFIValue>,
            LuaValue::LightUserData(value) => Box::new(value) as Box<dyn FFIValue>,
            LuaValue::UserData(value) => {
                Box::new(FFIUserData::new(lua, value)?) as Box<dyn FFIValue>
            }
            LuaValue::Error(value) => Box::new(value) as Box<dyn FFIValue>,
        };
        ffi_table.map.insert(key, value);
    }
    Ok(ffi_table)
}
