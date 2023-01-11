use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type Layer = crate::gfx::Layer;

impl LuaApiTable for Layer {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(|_lua, layer: u64| Ok(Self::new(layer)))?,
        )?;
        table.set("none", lua.create_function(|_lua, ()| Ok(Self::none()))?)?;
        table.set("all", lua.create_function(|_lua, ()| Ok(Self::all()))?)?;

        table.set(
            "has_overlap",
            lua.create_function(|_lua, (lhs, rhs): (Self, Self)| Ok(Self::has_overlap(lhs, rhs)))?,
        )?;

        Ok(table)
    }
}

impl LuaUserData for Layer {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("get", |_lua, this, ()| Ok(this.get()));
    }
}
