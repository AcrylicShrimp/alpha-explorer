use std::fmt::Display;

use mk::mlua::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapSize {
    pub width: u32,
    pub height: u32,
}

impl Display for MapSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MapSize({}x{})", self.width, self.height)
    }
}

impl LuaUserData for MapSize {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("width", |_lua, this| Ok(this.width));
        fields.add_field_method_get("height", |_lua, this| Ok(this.height));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        })
    }
}
