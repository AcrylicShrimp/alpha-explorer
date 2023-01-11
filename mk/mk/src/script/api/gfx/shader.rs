use mlua::prelude::*;

pub type Shader = crate::handles::ShaderHandle;

impl LuaUserData for Shader {}
