use downcast_rs::{impl_downcast, Downcast};
use mlua::prelude::*;
use std::any::Any;

pub trait Event
where
    Self: Downcast + ParamsToLuaTable,
{
    fn name(&self) -> &str;
    fn param(&self, param_name: &str) -> Option<&dyn Any>;
}

impl_downcast!(Event);

pub trait ParamsToLuaTable {
    fn params_to_lua_table<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaTable<'lua>>;
}

pub trait EventParamProvider
where
    Self: Event,
{
    fn get_param<T>(&self, param_name: &str) -> Option<&T>
    where
        T: Any,
    {
        self.param(param_name).and_then(|p| p.downcast_ref::<T>())
    }
}
