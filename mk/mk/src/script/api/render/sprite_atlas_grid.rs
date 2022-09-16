use crate::script::api::IntoShared;
use mlua::prelude::*;

define_shared_type!(SpriteAtlasGrid, crate::render::SpriteAtlasGrid);

impl LuaUserData for SpriteAtlasGrid {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("texture", |_lua, this| {
            Ok(this.texture().clone().into_shared())
        });
        fields.add_field_method_get("sprites", |_lua, this| {
            Ok(this
                .sprites()
                .iter()
                .cloned()
                .map(IntoShared::into_shared)
                .collect::<Vec<_>>())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

        methods.add_meta_method(LuaMetaMethod::Len, |_lua, this, ()| {
            Ok(this.sprites().len())
        });
        methods.add_meta_method(LuaMetaMethod::Index, |_lua, this, index: usize| {
            Ok(this.sprites()[index].clone().into_shared())
        });
    }
}
