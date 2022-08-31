use crate::script::api::ModuleType;
use rhai::Module;

mod ui_anchor;
mod ui_margin;

pub use ui_anchor::*;
pub use ui_margin::*;

pub struct UIModule;

impl ModuleType for UIModule {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        ui_anchor::UIAnchor::register(&mut sub_module);
        ui_margin::UIMargin::register(&mut sub_module);

        module.set_sub_module("ui", sub_module);
    }
}
