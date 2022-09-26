use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type Color = crate::render::Color;

impl LuaApiTable for Color {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "from_rgb",
            lua.create_function(|_lua, (r, g, b)| Ok(Self::from_rgb(r, g, b)))?,
        )?;
        table.set(
            "from_rgba",
            lua.create_function(|_lua, (r, g, b, a)| Ok(Self::from_rgba(r, g, b, a)))?,
        )?;
        table.set(
            "parse_hex",
            lua.create_function(|_lua, hex: LuaString| {
                Ok(Self::parse_hex(hex.to_str()?).map_err(|err| err.to_lua_err())?)
            })?,
        )?;
        table.set(
            "transparent",
            lua.create_function(|_lua, ()| Ok(Self::transparent()))?,
        )?;
        table.set("black", lua.create_function(|_lua, ()| Ok(Self::black()))?)?;
        table.set("red", lua.create_function(|_lua, ()| Ok(Self::red()))?)?;
        table.set("green", lua.create_function(|_lua, ()| Ok(Self::green()))?)?;
        table.set("blue", lua.create_function(|_lua, ()| Ok(Self::blue()))?)?;
        table.set(
            "yellow",
            lua.create_function(|_lua, ()| Ok(Self::yellow()))?,
        )?;
        table.set(
            "magenta",
            lua.create_function(|_lua, ()| Ok(Self::magenta()))?,
        )?;
        table.set("cyan", lua.create_function(|_lua, ()| Ok(Self::cyan()))?)?;
        table.set("white", lua.create_function(|_lua, ()| Ok(Self::white()))?)?;

        Ok(table)
    }
}

impl LuaUserData for Color {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("r", |_lua, this| Ok(this.r));
        fields.add_field_method_get("g", |_lua, this| Ok(this.g));
        fields.add_field_method_get("b", |_lua, this| Ok(this.b));
        fields.add_field_method_get("a", |_lua, this| Ok(this.a));

        fields.add_field_method_set("r", |_lua, this, r| {
            this.r = r;
            Ok(())
        });
        fields.add_field_method_set("g", |_lua, this, g| {
            this.r = g;
            Ok(())
        });
        fields.add_field_method_set("b", |_lua, this, b| {
            this.r = b;
            Ok(())
        });
        fields.add_field_method_set("a", |_lua, this, a| {
            this.r = a;
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

        methods.add_meta_function(LuaMetaMethod::Mul, |_lua, (lhs, rhs): (Self, Self)| {
            Ok(lhs * rhs)
        });
    }
}
