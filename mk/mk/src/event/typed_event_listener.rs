use crate::{script::ScriptManager, util::BoxId};
use anyhow::{Context, Result};
use mlua::prelude::*;
use std::any::{type_name, Any};

pub enum TypedEventListener<T>
where
    T: 'static,
{
    Native(BoxId<dyn FnMut(&T)>),
    Script(BoxId<LuaRegistryKey>),
}

impl<T> TypedEventListener<T>
where
    T: 'static,
{
    pub fn upcast(self) -> Option<BoxId<dyn Any>> {
        match self {
            TypedEventListener::Native(_) => None,
            TypedEventListener::Script(inner) => Some(inner.upcast()),
        }
    }

    pub fn hash(&self) -> usize {
        match self {
            TypedEventListener::Native(f) => f.hash(),
            TypedEventListener::Script(key) => key.hash(),
        }
    }

    pub fn listen<'lua, 't>(&mut self, script_mgr: &'lua ScriptManager, event: &'t T) -> Result<()>
    where
        T: Clone + ToLua<'lua>,
    {
        match self {
            Self::Native(f) => {
                f(event);
            }
            Self::Script(f) => {
                script_mgr
                    .call(
                        script_mgr
                            .lua()
                            .registry_value::<LuaFunction<'lua>>(&f)
                            .with_context(|| "the value is not valid lua function")?,
                        (event.clone(),),
                    )
                    .with_context(|| {
                        format!("failed to call an event handler of {}", type_name::<T>())
                    })?;
            }
        };
        Ok(())
    }
}
