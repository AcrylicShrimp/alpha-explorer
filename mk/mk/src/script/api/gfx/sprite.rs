use mlua::prelude::*;

pub type Sprite = crate::handles::SpriteHandle;

impl LuaUserData for Sprite {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("texture", |_lua, this| Ok(this.texture().clone()));
        fields.add_field_method_get("mapping", |_lua, this| Ok(this.mapping()));
        fields.add_field_method_get("slice", |_lua, this| Ok(this.slice()));
        fields.add_field_method_get("width", |_lua, this| Ok(this.width()));
        fields.add_field_method_get("height", |_lua, this| Ok(this.height()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });
    }
}
