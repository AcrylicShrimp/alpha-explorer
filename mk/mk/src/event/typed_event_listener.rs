use crate::{script::ScriptManager, util::BoxId};
use anyhow::{Context, Result};
use rhai::{FnPtr, GlobalRuntimeState, Identifier, NativeCallContext, Position};
use std::{any::type_name, ops::Deref};

pub enum TypedEventListener<T>
where
    T: 'static,
{
    Native(BoxId<dyn FnMut(&T)>),
    Script(BoxId<(FnPtr, ScriptFnContext)>),
}

pub struct ScriptFnContext {
    pub fn_name: String,
    pub source: Option<String>,
    pub global: GlobalRuntimeState<'static>,
    pub libs: Vec<Identifier>,
    pub pos: Position,
    pub level: usize,
}

impl<T> TypedEventListener<T>
where
    T: 'static,
{
    pub fn hash(&self) -> usize {
        match self {
            TypedEventListener::Native(f) => f.hash(),
            TypedEventListener::Script(key) => key.hash(),
        }
    }

    pub fn listen<'lua>(&mut self, script_mgr: &ScriptManager, event: &T) -> Result<()>
    where
        T: 'static + Send + Sync + Clone,
    {
        match self {
            Self::Native(f) => {
                f(event);
            }
            Self::Script(f) => {
                let module_cache_mgr = crate::engine::use_context().module_cache_mgr();
                let (f, ctx) = &(*f).deref();

                let libs = ctx
                    .libs
                    .iter()
                    .flat_map(|lib| module_cache_mgr.get_module(lib))
                    .collect::<Vec<_>>();
                let libs = libs.iter().map(|lib| lib.as_ref()).collect::<Vec<_>>();
                let context = NativeCallContext::new_with_all_fields(
                    script_mgr.engine(),
                    &ctx.fn_name,
                    ctx.source.as_ref(),
                    &ctx.global,
                    &libs,
                    ctx.pos,
                    ctx.level,
                );
                script_mgr
                    .call(f, &context, (event.clone(),))
                    .with_context(|| {
                        format!("failed to call an event handler of {}", type_name::<T>())
                    })?;
            }
        };
        Ok(())
    }
}
