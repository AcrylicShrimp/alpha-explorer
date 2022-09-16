use mlua::prelude::*;

define_shared_type!(Buffer, render::Buffer);

impl LuaUserData for Buffer {}
