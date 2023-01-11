use crate::script::api::{gfx::AlphaTileset, IntoShared, LuaApiTable};
use mlua::prelude::*;

pub type AlphaTilemap = crate::gfx::AlphaTilemap;

impl LuaApiTable for AlphaTilemap {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(
                |_lua,
                 (tile_width, tile_height, tile_count_x, tile_count_y, layer, tileset): (
                    _,
                    _,
                    _,
                    _,
                    _,
                    AlphaTileset,
                )| {
                    Ok(Self::new(
                        tile_width,
                        tile_height,
                        tile_count_x,
                        tile_count_y,
                        layer,
                        tileset.into_inner(),
                    ))
                },
            )?,
        )?;

        Ok(table)
    }
}

impl LuaUserData for AlphaTilemap {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("tile_width", |_lua, this| Ok(this.tile_width));
        fields.add_field_method_get("tile_height", |_lua, this| Ok(this.tile_height));
        fields.add_field_method_get("tile_count_x", |_lua, this| Ok(this.tile_count_x));
        fields.add_field_method_get("tile_count_y", |_lua, this| Ok(this.tile_count_y));
        fields.add_field_method_get("layer", |_lua, this| Ok(this.layer.clone()));
        fields.add_field_method_get("tileset", |_lua, this| {
            Ok(this.tileset.clone().into_shared())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });
    }
}
