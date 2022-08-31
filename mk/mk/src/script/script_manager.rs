use super::{build_module, ModuleCacheManager};
use crate::engine::use_context;
use anyhow::{anyhow, Context, Result};
use rhai::{
    module_resolvers::FileModuleResolver, Engine, EvalAltResult, FnPtr, FuncArgs, Module,
    ModuleResolver, NativeCallContext, Position, Scope, Shared, AST,
};
use std::path::{Path, PathBuf};

pub struct MkModuleResolver {
    mk_module: Shared<Module>,
    file_module_resolver: FileModuleResolver,
}

impl MkModuleResolver {
    pub fn new(mk_module: Shared<Module>, base_path: impl Into<PathBuf>) -> Self {
        Self {
            mk_module,
            file_module_resolver: FileModuleResolver::new_with_path(base_path),
        }
    }
}

impl ModuleResolver for MkModuleResolver {
    fn resolve(
        &self,
        engine: &Engine,
        source: Option<&str>,
        path: &str,
        pos: Position,
    ) -> Result<Shared<Module>, Box<EvalAltResult>> {
        if path == "mk" {
            Ok(self.mk_module.clone())
        } else {
            match self.file_module_resolver.resolve(engine, source, path, pos) {
                Ok(module) => {
                    use_context()
                        .module_cache_mgr_mut()
                        .set_module(module.id().unwrap().into(), module.clone());
                    Ok(module)
                }
                Err(err) => Err(err),
            }
        }
    }
}

struct CompiledModule {
    ast: AST,
    scope: Scope<'static>,
}

pub struct ScriptManager {
    engine: Engine,
    module: Option<CompiledModule>,
}

impl ScriptManager {
    pub fn new(module_cache_mgr: &mut ModuleCacheManager, base_path: impl AsRef<Path>) -> Self {
        let mk_module = Shared::new(build_module());
        module_cache_mgr.set_module(mk_module.id().unwrap().into(), mk_module.clone());

        let mut engine = Engine::new();
        engine.set_module_resolver(MkModuleResolver::new(mk_module, base_path.as_ref()));

        Self {
            engine,
            module: None,
        }
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn compile(&mut self, script: impl AsRef<str>) -> Result<()> {
        let scope = Scope::new();
        let ast = self
            .engine
            .compile_into_self_contained(&scope, script)
            .with_context(|| "failed to compile the entry script")?;

        self.module = Some(CompiledModule { ast, scope });
        Ok(())
    }

    pub fn execute(&mut self) -> Result<()> {
        let module = if let Some(module) = &mut self.module {
            module
        } else {
            return Err(anyhow!("no entry script compiled"));
        };

        self.engine
            .run_ast_with_scope(&mut module.scope, &module.ast)
            .with_context(|| "failed to execute the entry script")?;
        Ok(())
    }

    pub fn call<R>(&self, f: &FnPtr, ctx: &NativeCallContext, args: impl FuncArgs) -> Result<R>
    where
        R: Clone + Send + Sync + 'static,
    {
        f.call_within_context(ctx, args)
            .map_err(|err| {
                println!("{:?}", err);
                err
            })
            .with_context(|| "failed to call the function")
    }
}
