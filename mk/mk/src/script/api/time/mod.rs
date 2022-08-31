use crate::script::api::ModuleType;
use rhai::Module;

mod time;

pub use time::*;

pub struct TimeModule;

impl ModuleType for TimeModule {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        time::Time::register(&mut sub_module);

        module.set_sub_module("time", sub_module);
    }
}
