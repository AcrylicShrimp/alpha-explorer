use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type AlphaTile = crate::gfx::AlphaTile;

impl LuaApiTable for AlphaTile {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(
                |_lua, (fore_color, back_color, character): (_, _, LuaString)| {
                    Ok(Self::new(
                        fore_color,
                        back_color,
                        character.to_str()?.chars().next().unwrap_or(' '),
                    ))
                },
            )?,
        )?;

        Ok(table)
    }
}

impl LuaUserData for AlphaTile {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("fore_color", |_lua, this| Ok(this.fore_color));
        fields.add_field_method_get("back_color", |_lua, this| Ok(this.back_color));
        fields.add_field_method_get("character", |_lua, this| Ok(this.character.to_string()));

        fields.add_field_method_set("fore_color", |_lua, this, fore_color| {
            this.fore_color = fore_color;
            Ok(())
        });
        fields.add_field_method_set("back_color", |_lua, this, back_color| {
            this.back_color = back_color;
            Ok(())
        });
        fields.add_field_method_set("character", |_lua, this, character: LuaString| {
            this.character = character.to_str()?.chars().next().unwrap_or(' ');
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });
    }
}
