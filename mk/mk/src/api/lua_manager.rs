use crate::api::lua_api;
use crate::EngineContextWithoutSystemManager;
use mlua::prelude::*;
use std::path::Path;
use std::sync::Arc;

static mut CONTEXT: Option<Arc<EngineContextWithoutSystemManager>> = None;

pub struct LuaManager {
    lua: Lua,
}

impl LuaManager {
    pub fn new() -> Self {
        Self { lua: Lua::new() }
    }

    pub fn lua(&self) -> &Lua {
        &self.lua
    }

    pub fn init_lua<'lua, S: AsRef<str>>(
        &'lua self,
        context: Arc<EngineContextWithoutSystemManager>,
        api_prefix: S,
    ) -> Result<(), LuaError> {
        unsafe { CONTEXT = Some(context.clone()) };

        self.lua
            .globals()
            .set(api_prefix.as_ref(), lua_api(&self.lua)?)?;

        Ok(())
    }

    pub fn execute<P: AsRef<Path>, S: AsRef<str>>(
        &self,
        path: P,
        script: S,
    ) -> Result<(), LuaError> {
        self.lua
            .load(script.as_ref())
            .set_name(
                format!(
                    "@{}",
                    path.as_ref().as_os_str().to_str().unwrap_or("unknown")
                )
                .as_str(),
            )?
            .exec()
    }

    // pub fn execute_coroutine(
    //     &self,
    //     function: LuaFunction,
    //     args: LuaMultiValue,
    // ) -> LuaResult<Option<usize>> {
    //     let coroutine = self.lua.create_thread(function)?;
    //     let event = match coroutine.resume::<_, Option<LuaTable>>(args) {
    //         Ok(event) => match event {
    //             Some(event) => event,
    //             None => return Ok(None),
    //         },
    //         Err(err) => match err {
    //             LuaError::CoroutineInactive => return Ok(None),
    //             _ => return Err(err),
    //         },
    //     };
    //     let coroutine = BoxId::new(self.lua.create_registry_value(coroutine)?);
    //     let hash = coroutine.hash();
    //     use_context()
    //         .event_mgr()
    //         .dispatcher()
    //         .add_coroutine_listener_untyped(event.try_into()?, coroutine);

    //     Ok(Some(hash))
    // }

    // pub fn stop_coroutine(&self, hash: usize) {
    //     use_context()
    //         .event_mgr()
    //         .dispatcher()
    //         .remove_listener_untyped(hash);
    // }
}

// TODO: Move this to outer scope.
pub fn use_context() -> &'static Arc<EngineContextWithoutSystemManager> {
    return unsafe { &CONTEXT }.as_ref().unwrap();
}
