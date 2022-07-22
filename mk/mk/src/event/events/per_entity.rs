use legion::Entity;
use mlua::prelude::*;

// We're not going to expose lua bindings for this type.
#[derive(Debug, Clone)]
pub struct PerEntity<'lua> {
    pub entity: Entity,
    pub event: String,
    pub param: LuaMultiValue<'lua>,
}
