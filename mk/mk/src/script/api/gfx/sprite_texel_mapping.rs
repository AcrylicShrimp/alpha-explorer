use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type SpriteTexelMapping = crate::gfx::SpriteTexelMapping;

impl LuaApiTable for SpriteTexelMapping {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(|_lua, table: LuaTable| {
                Ok(Self::new(
                    table.get("x_min")?,
                    table.get("x_max")?,
                    table.get("y_min")?,
                    table.get("y_max")?,
                ))
            })?,
        )?;

        Ok(table)
    }
}

impl LuaUserData for SpriteTexelMapping {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("min", |lua, this| {
            let (x, y) = this.min();
            let table = lua.create_table()?;
            table.set("x", x)?;
            table.set("y", y)?;
            Ok(table)
        });
        fields.add_field_method_get("max", |lua, this| {
            let (x, y) = this.max();
            let table = lua.create_table()?;
            table.set("x", x)?;
            table.set("y", y)?;
            Ok(table)
        });
        fields.add_field_method_get("width", |_lua, this| Ok(this.width()));
        fields.add_field_method_get("height", |_lua, this| Ok(this.height()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

        methods.add_meta_function(LuaMetaMethod::Eq, |_lua, (lhs, rhs): (Self, Self)| {
            Ok(lhs == rhs)
        });
    }
}
