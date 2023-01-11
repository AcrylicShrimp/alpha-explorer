use mlua::prelude::*;

pub type Font = crate::handles::FontHandle;

impl LuaUserData for Font {}
