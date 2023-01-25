use super::Event;
use crate::script::{entity::Entity, FFIFunction};
use anyhow::{Error as AnyError, Result as AnyResult};
use mlua::prelude::*;
use std::{fmt::Display, hash::Hash, mem::discriminant, sync::Arc};
use thiserror::Error;

pub type NativeHandlerFunction = dyn Fn(Entity, &dyn Event) -> AnyResult<()>;

#[derive(Clone)]
pub enum EntityEventHandler {
    LuaEventHandler(FFIFunction),
    NativeEventHandler(Arc<NativeHandlerFunction>),
}

impl Display for EntityEventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityEventHandler::LuaEventHandler(handler) => {
                write!(f, "LuaEventHandler({:?})", handler.as_registry_key())
            }
            EntityEventHandler::NativeEventHandler(handler) => {
                write!(f, "NativeEventHandler({:p})", handler.as_ref() as *const _)
            }
        }
    }
}

impl LuaUserData for EntityEventHandler {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

        methods.add_meta_function(LuaMetaMethod::Eq, |_lua, (lhs, rhs): (Self, Self)| {
            Ok(lhs == rhs)
        });
    }
}

impl EntityEventHandler {
    pub fn lua(f: FFIFunction) -> Self {
        Self::LuaEventHandler(f)
    }

    pub fn native(f: impl Fn(Entity, &dyn Event) -> AnyResult<()> + 'static) -> Self {
        Self::NativeEventHandler(Arc::new(f))
    }

    pub fn handle<'lua>(
        &self,
        entity: Entity,
        event: &dyn Event,
        lua: &'lua Lua,
    ) -> EventHandlingResult<()> {
        match self {
            Self::LuaEventHandler(f) => {
                f.as_function(lua)?
                    .call((entity, event.name(), event.params_to_lua_table(lua)))?;
                Ok(())
            }
            Self::NativeEventHandler(f) => {
                f(entity, event)?;
                Ok(())
            }
        }
    }
}

impl PartialEq for EntityEventHandler {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LuaEventHandler(l0), Self::LuaEventHandler(r0)) => l0 == r0,
            (Self::NativeEventHandler(l0), Self::NativeEventHandler(r0)) => {
                l0.as_ref() as *const _ == r0.as_ref() as *const _
            }
            _ => false,
        }
    }
}

impl Eq for EntityEventHandler {}

impl Hash for EntityEventHandler {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
        match self {
            Self::LuaEventHandler(f) => f.hash(state),
            Self::NativeEventHandler(f) => (f.as_ref() as *const NativeHandlerFunction).hash(state),
        }
    }
}

#[derive(Error, Debug)]
pub enum EventHandlingError {
    #[error("error occurred while handling an event [lua handler]: {0}")]
    LuaError(#[from] LuaError),
    #[error("error occurred while handling an event [native handler]: {0}")]
    NativeError(#[from] AnyError),
}

pub type EventHandlingResult<T> = Result<T, EventHandlingError>;
