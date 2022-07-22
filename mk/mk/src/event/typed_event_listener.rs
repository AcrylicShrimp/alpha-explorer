use crate::util::BoxId;
use mlua::prelude::*;

pub enum TypedEventListener<T>
where
    T: 'static,
{
    Native(BoxId<dyn FnMut(&T)>),
    LuaFunction(BoxId<LuaRegistryKey>),
}

impl<T> TypedEventListener<T>
where
    T: 'static,
{
    pub fn hash(&self) -> usize {
        match self {
            TypedEventListener::Native(f) => f.hash(),
            TypedEventListener::LuaFunction(key) => key.hash(),
        }
    }

    pub fn listen<'lua>(&mut self, lua: &'lua Lua, event: &T) -> LuaResult<()>
    where
        T: Clone + ToLua<'lua>,
    {
        Ok(match self {
            Self::Native(f) => {
                f(event);
            }
            Self::LuaFunction(key) => {
                lua.registry_value::<LuaFunction>(key)?
                    .call::<_, ()>(event.clone().to_lua(lua)?)?;
            }
        })
    }
}
