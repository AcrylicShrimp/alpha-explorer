use mlua::prelude::*;

define_shared_type!(Font, fontdue::Font);

impl LuaUserData for Font {}
