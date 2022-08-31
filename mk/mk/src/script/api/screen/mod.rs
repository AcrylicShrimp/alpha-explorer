use crate::script::api::ModuleType;
use rhai::Module;

mod screen;

pub use screen::*;

pub struct ScreenModule;

impl ModuleType for ScreenModule {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        screen::Screen::register(&mut sub_module);

        module.set_sub_module("screen", sub_module);
    }
}
