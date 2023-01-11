use crate::script::api::{IntoShared, LuaApiTable};
use mlua::prelude::*;

define_shared_type!(AlphaTileset, crate::gfx::AlphaTileset);

impl LuaApiTable for AlphaTileset {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(|_lua, tiles| Ok(Inner::new(tiles).into_shared()))?,
        )?;

        Ok(table)
    }
}

impl LuaUserData for AlphaTileset {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("tiles", |_lua, this| Ok(this.tiles.clone()))
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.0.to_string())
        })
    }
}
