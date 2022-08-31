use crate::script::api::ModuleType;
use rhai::Module;

mod size;
mod vec2;

pub use size::*;
pub use vec2::*;

pub struct StructureModule;

impl ModuleType for StructureModule {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        size::Size::register(&mut sub_module);
        vec2::Vec2::register(&mut sub_module);

        module.set_sub_module("structure", sub_module);
    }
}
