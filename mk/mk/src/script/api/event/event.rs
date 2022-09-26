use crate::engine::use_context;
use mlua::prelude::*;
use std::{any::Any, marker::PhantomData};

pub struct Event<T>
where
    T: 'static + Any + Clone + for<'lua> ToLua<'lua>,
{
    _t: PhantomData<T>,
}

impl<T> LuaUserData for Event<T>
where
    T: 'static + Any + Clone + for<'lua> ToLua<'lua>,
{
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("listen", |lua, _this, handler: LuaFunction| {
            let event_mgr = use_context().event_mgr();
            Ok(event_mgr
                .dispatcher()
                .add_listener::<T>(crate::event::TypedEventListener::Script(
                    crate::util::BoxId::new(lua.create_registry_value(handler)?),
                )))
        });
        methods.add_method("unlisten", |lua, _this, handler: usize| {
            let event_mgr = use_context().event_mgr();
            if let Some(listener) = event_mgr.dispatcher().remove_listener::<T>(handler) {
                if let Ok(handler) = listener.into_inner().downcast::<mlua::RegistryKey>() {
                    lua.remove_registry_value(*handler)?;
                }
            }
            Ok(())
        });
    }
}
