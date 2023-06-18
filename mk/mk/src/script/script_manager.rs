use super::{LuaApiTable, Module};
use anyhow::{Context, Result};
use mlua::prelude::*;

pub trait LuaCallable<'lua, A, R> {
    fn call(&self, lua: &'lua Lua, args: A) -> Result<R>
    where
        A: ToLuaMulti<'lua>,
        R: FromLuaMulti<'lua>;
}

impl<'lua, A, R> LuaCallable<'lua, A, R> for LuaFunction<'lua>
where
    A: ToLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    fn call(&self, _lua: &'lua Lua, args: A) -> Result<R>
    where
        A: ToLuaMulti<'lua>,
        R: FromLuaMulti<'lua>,
    {
        self.call(args)
            .with_context(|| "failed to call lua function")
    }
}

impl<'lua, A, R> LuaCallable<'lua, A, R> for LuaThread<'lua>
where
    A: ToLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    fn call(&self, _lua: &'lua Lua, args: A) -> Result<R>
    where
        A: ToLuaMulti<'lua>,
        R: FromLuaMulti<'lua>,
    {
        self.resume(args)
            .with_context(|| "failed to call lua coroutine")
    }
}

pub struct ScriptManager {
    lua: Lua,
}

impl ScriptManager {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        lua.globals()
            .raw_set("mk", Module::create_api_table(&lua)?)?;
        Ok(Self { lua })
    }

    pub fn lua(&self) -> &Lua {
        &self.lua
    }

    pub fn append_api_table<T>(&self, name: impl AsRef<str>) -> Result<()>
    where
        T: LuaApiTable,
    {
        self.lua
            .globals()
            .raw_set(name.as_ref(), T::create_api_table(&self.lua)?)
            .with_context(|| "unable to append api table")
    }

    pub fn execute(&self, chunk: impl AsRef<str>) -> Result<()> {
        self.lua
            .load(chunk.as_ref())
            .set_name(chunk.as_ref())?
            .exec()
            .with_context(|| "unable to execute lua chunk")
    }

    pub fn call<'lua, A, R>(
        &'lua self,
        callable: impl LuaCallable<'lua, A, R>,
        args: A,
    ) -> Result<R>
    where
        A: ToLuaMulti<'lua>,
        R: FromLuaMulti<'lua>,
    {
        callable.call(&self.lua, args)
    }
}
