use mlua::prelude::*;
use mlua::UserData;
use std::any::type_name;

#[derive(Debug, Clone, Copy)]
pub struct PreUpdate {
    pub dt: f32,
}

impl_event_type_lua_api!(PreUpdate);

impl UserData for PreUpdate {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Index, |_lua, this, value: String| {
            Ok(match value.as_str() {
                "dt" => this.dt,
                _ => {
                    return Err(format!(
                        "property '{}' is not exists on the '{}'",
                        value,
                        type_name::<Self>()
                    )
                    .to_lua_err())
                }
            })
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Update {
    pub dt: f32,
}

impl_event_type_lua_api!(Update);

impl UserData for Update {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Index, |_lua, this, value: String| {
            Ok(match value.as_str() {
                "dt" => this.dt,
                _ => {
                    return Err(format!(
                        "property '{}' is not exists on the '{}'",
                        value,
                        type_name::<Self>()
                    )
                    .to_lua_err())
                }
            })
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PostUpdate {
    pub dt: f32,
}

impl_event_type_lua_api!(PostUpdate);

impl UserData for PostUpdate {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Index, |_lua, this, value: String| {
            Ok(match value.as_str() {
                "dt" => this.dt,
                _ => {
                    return Err(format!(
                        "property '{}' is not exists on the '{}'",
                        value,
                        type_name::<Self>()
                    )
                    .to_lua_err())
                }
            })
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PreRender {
    pub dt: f32,
}

impl_event_type_lua_api!(PreRender);

impl UserData for PreRender {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Index, |_lua, this, value: String| {
            Ok(match value.as_str() {
                "dt" => this.dt,
                _ => {
                    return Err(format!(
                        "property '{}' is not exists on the '{}'",
                        value,
                        type_name::<Self>()
                    )
                    .to_lua_err())
                }
            })
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PostRender {
    pub dt: f32,
}

impl_event_type_lua_api!(PostRender);

impl UserData for PostRender {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Index, |_lua, this, value: String| {
            Ok(match value.as_str() {
                "dt" => this.dt,
                _ => {
                    return Err(format!(
                        "property '{}' is not exists on the '{}'",
                        value,
                        type_name::<Self>()
                    )
                    .to_lua_err())
                }
            })
        });
    }
}
