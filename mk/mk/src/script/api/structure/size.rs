use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type Size = crate::structure::Size;

impl LuaApiTable for Size {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(|_lua, (width, height)| Ok(Self::new(width, height)))?,
        )?;
        table.set("zero", lua.create_function(|_lua, ()| Ok(Self::zero()))?)?;
        table.set("one", lua.create_function(|_lua, ()| Ok(Self::one()))?)?;

        Ok(table)
    }
}

impl LuaUserData for Size {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("width", |_lua, this| Ok(this.width));
        fields.add_field_method_get("height", |_lua, this| Ok(this.height));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

        methods.add_meta_function(LuaMetaMethod::Mul, |_lua, (lhs, rhs): (Self, f32)| {
            Ok(lhs * rhs)
        });
        methods.add_meta_function(LuaMetaMethod::Mul, |_lua, (lhs, rhs): (f32, Self)| {
            Ok(lhs * rhs)
        });

        methods.add_meta_function(LuaMetaMethod::Div, |_lua, (lhs, rhs): (Self, f32)| {
            Ok(lhs / rhs)
        });

        methods.add_meta_function(LuaMetaMethod::Unm, |_lua, lhs: Self| Ok(-lhs));

        methods.add_method("area", |_lua, this, ()| Ok(this.area()));
    }
}
