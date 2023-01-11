use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type SpriteSlice = crate::gfx::SpriteSlice;

impl LuaApiTable for SpriteSlice {
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

impl LuaUserData for SpriteSlice {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

        methods.add_meta_function(LuaMetaMethod::Eq, |_lua, (lhs, rhs): (Self, Self)| {
            Ok(lhs == rhs)
        });
    }
}
