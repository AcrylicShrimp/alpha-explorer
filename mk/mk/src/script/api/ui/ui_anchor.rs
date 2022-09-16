use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type UIAnchor = crate::ui::UIAnchor;

impl LuaApiTable for UIAnchor {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(|_lua, (min, max)| Ok(Self::new(min, max)))?,
        )?;
        table.set("full", lua.create_function(|_lua, ()| Ok(Self::full()))?)?;

        Ok(table)
    }
}

impl LuaUserData for UIAnchor {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("min", |_lua, this| Ok(this.min));
        fields.add_field_method_get("max", |_lua, this| Ok(this.max));

        fields.add_field_method_set("min", |_lua, this, min| {
            this.min = min;
            Ok(())
        });
        fields.add_field_method_set("max", |_lua, this, max| {
            this.max = max;
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });
    }
}
