use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type UIMargin = crate::ui::UIMargin;

impl LuaApiTable for UIMargin {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(|_lua, (left, right, top, bottom)| {
                Ok(Self::new(left, right, top, bottom))
            })?,
        )?;
        table.set(
            "from_size",
            lua.create_function(|_lua, (pivot, position, size)| {
                Ok(Self::from_size(pivot, position, size))
            })?,
        )?;
        table.set("zero", lua.create_function(|_lua, ()| Ok(Self::zero()))?)?;

        Ok(table)
    }
}

impl LuaUserData for UIMargin {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("left", |_lua, this| Ok(this.left));
        fields.add_field_method_get("right", |_lua, this| Ok(this.right));
        fields.add_field_method_get("top", |_lua, this| Ok(this.top));
        fields.add_field_method_get("bottom", |_lua, this| Ok(this.bottom));

        fields.add_field_method_set("left", |_lua, this, left| {
            this.left = left;
            Ok(())
        });
        fields.add_field_method_set("right", |_lua, this, right| {
            this.right = right;
            Ok(())
        });
        fields.add_field_method_set("top", |_lua, this, top| {
            this.top = top;
            Ok(())
        });
        fields.add_field_method_set("bottom", |_lua, this, bottom| {
            this.bottom = bottom;
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });
    }
}
