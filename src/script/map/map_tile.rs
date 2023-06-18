use mk::{gfx::Color, mlua::prelude::*};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct MapTile {
    pub display: char,
    pub fore_color: Color,
    pub back_color: Color,
    pub movable: bool,
}

impl Display for MapTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MapTile({}, movable={})", self.display, self.movable)
    }
}

impl LuaUserData for MapTile {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("display", |_lua, this| Ok(this.display.to_string()));
        fields.add_field_method_get("fore_color", |_lua, this| Ok(this.fore_color));
        fields.add_field_method_get("back_color", |_lua, this| Ok(this.back_color));
        fields.add_field_method_get("movable", |_lua, this| Ok(this.movable));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
