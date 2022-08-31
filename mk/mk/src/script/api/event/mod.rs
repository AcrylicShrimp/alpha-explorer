use crate::script::api::ModuleType;
use rhai::Module;

macro_rules! impl_event_type {
    ($module:ident, $this:ident) => {
        $module.set_custom_type::<Self>(stringify!($this));

        $module.set_sub_module(stringify!($this), {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn(
                "listen",
                |context: rhai::NativeCallContext, handler: rhai::FnPtr| {
                    panic!("{:?}", context.namespaces());
                    let event_mgr = crate::engine::use_context().event_mgr();
                    Ok(event_mgr.dispatcher().add_listener::<Self>(
                        crate::event::TypedEventListener::Script(crate::util::BoxId::new((
                            handler,
                            crate::event::ScriptFnContext {
                                fn_name: context.fn_name().to_string(),
                                source: context.source().map(|s| s.to_string()),
                                global: unsafe {
                                    std::mem::transmute::<_, rhai::GlobalRuntimeState<'static>>(
                                        context.global_runtime_state().unwrap().clone(),
                                    )
                                },
                                libs: context
                                    .iter_namespaces()
                                    .filter_map(|lib| lib.id().map(|id| id.into()))
                                    .collect(),
                                pos: context.position(),
                                level: context.call_level(),
                            },
                        ))),
                    ))
                },
            );
            sub_module.set_native_fn("unlisten", |handler: usize| {
                let event_mgr = crate::engine::use_context().event_mgr();
                event_mgr.dispatcher().remove_listener::<Self>(handler);
                Ok(())
            });

            sub_module
        });
    };
}

mod diagnostic;
mod input;
mod lifecycles;
mod per_entity;

pub use diagnostic::*;
pub use input::*;
pub use lifecycles::*;
pub use per_entity::*;

pub struct EventModule;

impl ModuleType for EventModule {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        diagnostic::DiagnosticLevel::register(&mut sub_module);
        diagnostic::SubDiagnostic::register(&mut sub_module);
        diagnostic::Diagnostic::register(&mut sub_module);
        input::KeyDownEvent::register(&mut sub_module);
        input::KeyUpEvent::register(&mut sub_module);
        input::PointerEnter::register(&mut sub_module);
        input::PointerExit::register(&mut sub_module);
        input::PointerMove::register(&mut sub_module);
        input::PointerDown::register(&mut sub_module);
        input::PointerUp::register(&mut sub_module);
        lifecycles::PreUpdate::register(&mut sub_module);
        lifecycles::Update::register(&mut sub_module);
        lifecycles::PostUpdate::register(&mut sub_module);
        lifecycles::PreRender::register(&mut sub_module);
        lifecycles::PostRender::register(&mut sub_module);

        module.set_sub_module("Event", sub_module);
    }
}
