use mlua::prelude::*;

pub type Texture = crate::handles::TextureHandle;

impl LuaUserData for Texture {
    // fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    //     fields.add_field_method_get("width", |_lua, this| Ok(this.width()));
    //     fields.add_field_method_get("height", |_lua, this| Ok(this.height()));
    // }
}
