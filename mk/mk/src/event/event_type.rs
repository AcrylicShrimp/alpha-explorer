use mlua::prelude::*;
use std::any::TypeId;
use std::mem::transmute;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventType(TypeId);

impl<'lua> FromLua<'lua> for EventType {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        Ok(Self(match value {
            LuaValue::Integer(integer) => unsafe { transmute(integer) },
            _ => panic!("invalid event type"),
        }))
    }
}

impl<'lua> ToLua<'lua> for EventType {
    fn to_lua(self, _lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::Integer(unsafe { transmute(self.0) }))
    }
}

impl From<TypeId> for EventType {
    fn from(type_id: TypeId) -> Self {
        Self(type_id)
    }
}

impl From<EventType> for TypeId {
    fn from(event_type: EventType) -> TypeId {
        event_type.0
    }
}
