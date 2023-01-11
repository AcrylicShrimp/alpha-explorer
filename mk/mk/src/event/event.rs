use mlua::prelude::*;
use std::any::{Any, TypeId};

pub trait Event
where
    Self: for<'lua> ToLua<'lua>,
{
    fn type_id() -> TypeId;
    fn name() -> &'static str;
    fn param(&self, param_name: &str) -> Option<&dyn Any>;
}
