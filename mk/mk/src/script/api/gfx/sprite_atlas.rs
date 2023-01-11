use crate::script::api::IntoShared;
use mlua::prelude::*;

define_shared_type!(SpriteAtlas, crate::gfx::SpriteAtlas);

impl LuaUserData for SpriteAtlas {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("texture", |_lua, this| {
            Ok(this.texture().clone().into_shared())
        });
        fields.add_field_method_get("sprites", |_lua, this| {
            Ok(_lua.create_table_from(
                this.sprites()
                    .iter()
                    .map(|(name, sprite)| (name.clone(), sprite.clone().into_shared())),
            )?)
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

        methods.add_meta_method(LuaMetaMethod::Len, |_lua, this, ()| {
            Ok(this.sprites().len())
        });
        methods.add_meta_method(LuaMetaMethod::Index, |_lua, this, index: LuaString| {
            Ok(this
                .sprites()
                .get(index.to_str()?)
                .cloned()
                .map(IntoShared::into_shared))
        });
    }
}
