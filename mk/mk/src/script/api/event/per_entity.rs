use mlua::prelude::*;
use std::mem::transmute;

#[derive(Clone)]
pub struct PerEntity<T>
where
    T: 'static,
{
    pub entity: specs::Entity,
    pub event: String,
    pub param: T,
}

impl<'_lua, T> LuaUserData for PerEntity<T>
where
    T: Clone + ToLua<'_lua>,
{
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("entity", |_lua, this| {
            Ok(crate::script::entity::Entity::new(this.entity))
        });
        fields.add_field_method_get("event", |_lua, this| Ok(this.event.clone()));
        fields.add_field_method_get("param", |lua, this| {
            let value = this
                .param
                .clone()
                .to_lua(unsafe { transmute::<_, &'_lua Lua>(lua) })?;
            let value = unsafe { transmute::<_, LuaValue<'lua>>(value) };
            Ok(value)
        });
    }
}
