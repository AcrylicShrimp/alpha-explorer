use mlua::prelude::*;

define_shared_type!(Texture, render::Texture);

impl LuaUserData for Texture {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("width", |_lua, this| Ok(this.width()));
        fields.add_field_method_get("height", |_lua, this| Ok(this.height()));
    }
}
