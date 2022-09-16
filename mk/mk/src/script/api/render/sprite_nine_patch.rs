use crate::script::api::IntoShared;
use mlua::prelude::*;

define_shared_type!(SpriteNinePatch, crate::render::SpriteNinePatch);

impl LuaUserData for SpriteNinePatch {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("texture", |_lua, this| {
            Ok(this.texture().clone().into_shared())
        });
        fields.add_field_method_get("sprite_lt", |_lua, this| {
            Ok(this.sprite_lt().clone().into_shared())
        });
        fields.add_field_method_get("sprite_ct", |_lua, this| {
            Ok(this.sprite_lt().clone().into_shared())
        });
        fields.add_field_method_get("sprite_rt", |_lua, this| {
            Ok(this.sprite_lt().clone().into_shared())
        });
        fields.add_field_method_get("sprite_lm", |_lua, this| {
            Ok(this.sprite_lt().clone().into_shared())
        });
        fields.add_field_method_get("sprite_cm", |_lua, this| {
            Ok(this.sprite_lt().clone().into_shared())
        });
        fields.add_field_method_get("sprite_rm", |_lua, this| {
            Ok(this.sprite_lt().clone().into_shared())
        });
        fields.add_field_method_get("sprite_lb", |_lua, this| {
            Ok(this.sprite_lt().clone().into_shared())
        });
        fields.add_field_method_get("sprite_cb", |_lua, this| {
            Ok(this.sprite_lt().clone().into_shared())
        });
        fields.add_field_method_get("sprite_rb", |_lua, this| {
            Ok(this.sprite_lt().clone().into_shared())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });
    }
}
