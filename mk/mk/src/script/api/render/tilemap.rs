use crate::script::api::{render::SpriteAtlasGrid, IntoShared, LuaApiTable};
use mlua::prelude::*;

define_shared_type!(Tilemap, crate::render::Tilemap);

impl LuaApiTable for Tilemap {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(
                |_lua,
                 (tile_width, tile_height, tile_count_x, tile_count_y, layers, palette): (
                    _,
                    _,
                    _,
                    _,
                    _,
                    SpriteAtlasGrid,
                )| {
                    Ok(Self::new(crate::render::Tilemap::new(
                        tile_width,
                        tile_height,
                        tile_count_x,
                        tile_count_y,
                        layers,
                        palette.into_inner(),
                    )))
                },
            )?,
        )?;

        Ok(table)
    }
}

impl LuaUserData for Tilemap {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("tile_width", |_lua, this| Ok(this.tile_width));
        fields.add_field_method_get("tile_height", |_lua, this| Ok(this.tile_height));
        fields.add_field_method_get("tile_count_x", |_lua, this| Ok(this.tile_count_x));
        fields.add_field_method_get("tile_count_y", |_lua, this| Ok(this.tile_count_y));
        fields.add_field_method_get("layers", |_lua, this| Ok(this.layers.clone()));
        fields.add_field_method_get("palette", |_lua, this| {
            Ok(this.palette.clone().into_shared())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });
    }
}
