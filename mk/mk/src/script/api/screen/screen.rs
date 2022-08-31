use crate::{engine::use_context, script::api::ModuleType, structure::Size};
use rhai::Module;

pub struct Screen;

impl ModuleType for Screen {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        sub_module.set_native_fn("size", || {
            let screen_mgr = use_context().screen_mgr();
            Ok(Size::new(
                screen_mgr.width() as f32,
                screen_mgr.height() as f32,
            ))
        });
        sub_module.set_native_fn("physical_size", || {
            let screen_mgr = use_context().screen_mgr();
            Ok(Size::new(
                screen_mgr.physical_width() as f32,
                screen_mgr.physical_height() as f32,
            ))
        });
        sub_module.set_native_fn("scale_factor", || {
            let screen_mgr = use_context().screen_mgr();
            Ok(screen_mgr.scale_factor())
        });

        module.set_sub_module("Screen", sub_module);
    }
}
