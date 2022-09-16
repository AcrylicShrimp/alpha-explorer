use mlua::prelude::*;

define_shared_type!(Shader, render::Shader);

impl LuaUserData for Shader {}
