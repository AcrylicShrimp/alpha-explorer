use crate::script::api::LuaApiTable;
use mlua::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct PreUpdate {
    pub dt: f64,
}

impl LuaApiTable for PreUpdate {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for PreUpdate {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("dt", |_lua, this| Ok(this.dt));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Update {
    pub dt: f64,
}

impl LuaApiTable for Update {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for Update {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("dt", |_lua, this| Ok(this.dt));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PostUpdate {
    pub dt: f64,
}

impl LuaApiTable for PostUpdate {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for PostUpdate {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("dt", |_lua, this| Ok(this.dt));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PreRender {
    pub dt: f64,
}

impl LuaApiTable for PreRender {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for PreRender {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("dt", |_lua, this| Ok(this.dt));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PostRender {
    pub dt: f64,
}

impl LuaApiTable for PostRender {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for PostRender {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("dt", |_lua, this| Ok(this.dt));
    }
}
